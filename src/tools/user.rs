use serde::{Deserialize, Serialize};
use serde_json::json;

use anchor_lang::InstructionData;
use drift_rs::drift_idl;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};

#[derive(Debug, thiserror::Error)]
#[error("Drift transaction error")]
pub struct DriftError;

#[derive(Deserialize, Serialize)]
pub struct DepositArgs {
    pub amount: u64,
    pub spot_market_index: u16,
    pub user_token_account: String,
    pub reduce_only: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct WithdrawArgs {
    pub amount: u64,
    pub spot_market_index: u16,
    pub user_token_account: String,
    pub reduce_only: Option<bool>,
}

pub struct Deposit;
impl Deposit {
    pub async fn get_tx_data(args: DepositArgs) -> Result<String, DriftError> {
        let instruction_data = InstructionData::data(&drift_idl::instructions::Deposit {
            market_index: args.spot_market_index,
            amount: args.amount,
            reduce_only: args.reduce_only.unwrap_or(false),
        });
        Ok(hex::encode(instruction_data))
    }
}

impl Tool for Deposit {
    const NAME: &'static str = "deposit";

    type Error = DriftError;
    type Args = DepositArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "deposit".to_string(),
            description: "Generate transaction data for a Drift deposit.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "amount": { "type": "number", "description": "Amount to deposit" },
                    "spot_market_index": { "type": "integer", "description": "Market index" },
                    "user_token_account": { "type": "string", "description": "User's token account pubkey" },
                    "reduce_only": { "type": "boolean", "description": "Reduce-only flag" }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Self::get_tx_data(args).await
    }
}

pub struct Withdraw;
impl Withdraw {
    pub async fn get_tx_data(args: WithdrawArgs) -> Result<String, DriftError> {
        let instruction_data = InstructionData::data(&drift_idl::instructions::Withdraw {
            market_index: args.spot_market_index,
            amount: args.amount,
            reduce_only: args.reduce_only.unwrap_or(false),
        });
        Ok(hex::encode(instruction_data))
    }
}

impl Tool for Withdraw {
    const NAME: &'static str = "withdraw";

    type Error = DriftError;
    type Args = WithdrawArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "withdraw".to_string(),
            description: "Generate transaction data for a Drift withdrawal.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "amount": { "type": "number", "description": "Amount to withdraw" },
                    "spot_market_index": { "type": "integer", "description": "Market index" },
                    "user_token_account": { "type": "string", "description": "User's token account pubkey" },
                    "reduce_only": { "type": "boolean", "description": "Reduce-only flag" }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Self::get_tx_data(args).await
    }
}
