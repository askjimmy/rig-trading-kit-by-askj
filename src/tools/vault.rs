use std::{collections::HashMap, env, str::FromStr};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::nonblocking::rpc_client::RpcClient;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use drift_rs::Pubkey;

use crate::tools::shared::*;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ContractState {
    pub name: [u8; 32],
    pub pubkey: Pubkey,
    pub manager: Pubkey,
    pub token_account: Pubkey,
    pub user_stats: Pubkey,
    pub user: Pubkey,
    pub delegate: Pubkey,
    pub liquidation_delegate: Pubkey,
    pub user_shares: u128,
    pub total_shares: u128,
    pub last_fee_update_ts: i64,
    pub liquidation_start_ts: i64,
    pub redeem_period: i64,
    pub total_withdraw_requested: u64,
    pub max_tokens: u64,
    pub management_fee: i64,
    pub init_ts: i64,
    pub net_deposits: i64,
    pub manager_net_deposits: i64,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub manager_total_deposits: u64,
    pub manager_total_withdraws: u64,
    pub manager_total_fee: i64,
    pub manager_total_profit_share: u64,
    pub min_deposit_amount: u64,
    pub last_manager_withdraw_request: WithdrawRequest,
    pub shares_base: u32,
    pub profit_share: u32,
    pub hurdle_rate: u32,
    pub spot_market_index: u16,
    pub bump: u8,
    pub permissioned: bool,
    pub padding: [u64; 8],
    pub extra_padding: [u8; 8],
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct WithdrawRequest {
    pub shares: u128,
    pub value: u64,
    pub ts: i64,
}

#[derive(Deserialize, Serialize)]
pub struct DriftVaultInfoArgs {
    pub vault_address: Option<String>,
    pub requested_fields: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct DriftVaultInfo;
impl DriftVaultInfo {
    pub async fn fetch_vault_info(args: DriftVaultInfoArgs) -> Result<String, DriftDataError> {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let agent_vault: String = env::var("AGENT_VAULT").unwrap();
        let vault = &args.vault_address.unwrap_or(agent_vault);
        let vault_pubkey = Pubkey::from_str(vault).map_err(|_| DriftDataError { 
            message: "Failed to get vault pubkey".to_string()
        })?;
        let account_data = rpc_client.get_account_data(&vault_pubkey).await.map_err(|_| DriftDataError { 
            message: "Failed to get_account_data".to_string()
        })?;
        let contract_state: ContractState = ContractState::try_from_slice(&account_data)
            .map_err(|_| DriftDataError { 
                message: "Failed to get contract state".to_string()
            })?;

        let mut fields_map: HashMap<String, String> = HashMap::new();
        let name_ascii: String = contract_state.name.iter()
            .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
            .map(|&b| b as char)
            .collect();

        fields_map.insert("vault_address".to_string(), vault.clone());
        fields_map.insert("name".to_string(), format!("{:?}", name_ascii));
        fields_map.insert("manager".to_string(), format!("{:?}", contract_state.manager));
        fields_map.insert("token_account".to_string(), format!("{:?}", contract_state.token_account));
        fields_map.insert("user_stats".to_string(), format!("{:?}", contract_state.user_stats));
        fields_map.insert("user".to_string(), format!("{:?}", contract_state.user));
        fields_map.insert("delegate".to_string(), format!("{:?}", contract_state.delegate));
        fields_map.insert("liquidation_delegate".to_string(), format!("{:?}", contract_state.liquidation_delegate));
        fields_map.insert("user_shares".to_string(), contract_state.user_shares.to_string());
        fields_map.insert("total_shares".to_string(), contract_state.total_shares.to_string());
        fields_map.insert("last_fee_update_ts".to_string(), contract_state.last_fee_update_ts.to_string());
        fields_map.insert("liquidation_start_ts".to_string(), contract_state.liquidation_start_ts.to_string());
        fields_map.insert("redeem_period".to_string(), contract_state.redeem_period.to_string());
        fields_map.insert("total_withdraw_requested".to_string(), contract_state.total_withdraw_requested.to_string());
        fields_map.insert("max_tokens".to_string(), contract_state.max_tokens.to_string());
        fields_map.insert("management_fee".to_string(), contract_state.management_fee.to_string());
        fields_map.insert("init_ts".to_string(), contract_state.init_ts.to_string());
        fields_map.insert("net_deposits".to_string(), contract_state.net_deposits.to_string());
        fields_map.insert("manager_net_deposits".to_string(), contract_state.manager_net_deposits.to_string());
        fields_map.insert("total_deposits".to_string(), contract_state.total_deposits.to_string());
        fields_map.insert("total_withdraws".to_string(), contract_state.total_withdraws.to_string());
        fields_map.insert("manager_total_deposits".to_string(), contract_state.manager_total_deposits.to_string());
        fields_map.insert("manager_total_withdraws".to_string(), contract_state.manager_total_withdraws.to_string());
        fields_map.insert("manager_total_fee".to_string(), contract_state.manager_total_fee.to_string());
        fields_map.insert("manager_total_profit_share".to_string(), contract_state.manager_total_profit_share.to_string());
        fields_map.insert("min_deposit_amount".to_string(), contract_state.min_deposit_amount.to_string());
        fields_map.insert("shares_base".to_string(), contract_state.shares_base.to_string());
        fields_map.insert("profit_share".to_string(), contract_state.profit_share.to_string());
        fields_map.insert("hurdle_rate".to_string(), contract_state.hurdle_rate.to_string());
        fields_map.insert("spot_market_index".to_string(), contract_state.spot_market_index.to_string());
        fields_map.insert("bump".to_string(), contract_state.bump.to_string());
        fields_map.insert("permissioned".to_string(), contract_state.permissioned.to_string());

        let mut result = String::new();
        if let Some(requested_fields) = args.requested_fields {
            for field in requested_fields {
                if let Some(value) = fields_map.get(&field) {
                    result.push_str(&format!("{}: {}\n", field, value));
                }
            }
        } else {
            for (key, value) in fields_map.iter() {
                result.push_str(&format!("{}: {}\n", key, value));
            }
        }

        Ok(result)
    }
}

impl Tool for DriftVaultInfo {
    const NAME: &'static str = "drift_vault_info";

    type Error = DriftDataError;
    type Args = DriftVaultInfoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_vault_info".to_string(),
            description: "Fetch all Drift vault information including vault_address, name, manager, token_account, user_stats, user, delegate, liquidation_delegate, user_shares, total_shares, last_fee_update_ts, liquidation_start_ts, redeem_period, total_withdraw_requested, max_tokens, management_fee, init_ts, net_deposits, manager_net_deposits, total_deposits, total_withdraws, manager_total_deposits, manager_total_withdraws, manager_total_fee, manager_total_profit_share, min_deposit_amount, shares_base, profit_share, hurdle_rate, spot_market_index, bump, permissioned".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "vault_address": { "type": "string", "description": "The public key of the vault" },
                    "requested_fields": { "type": "array", "items": { "type": "string" }, "description": "List of fields to retrieve" }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        tokio::task::spawn_blocking(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(DriftVaultInfo::fetch_vault_info(args))
        })
        .await
        .map_err(|_| DriftDataError { 
            message: "Failed to call fetch_vault_info".to_string()
        })?
    }
    
}