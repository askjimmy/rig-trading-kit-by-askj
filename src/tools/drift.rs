use std::{env, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_json::json;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use drift_rs::{types::{Context, MarketId, Order, OrderStatus, PerpPosition, SpotPosition}, DriftClient, Wallet};

use crate::data::*;
use crate::tools::shared::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DriftInfoArgs {
    pub market_index: u16,
}

#[derive(Deserialize, Serialize)]
pub struct DriftInfo;

impl Tool for DriftInfo {
    const NAME: &'static str = "drift_info";

    type Error = DriftDataError;
    type Args = DriftInfoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_info".to_string(),
            description: "Fetch Drift market information and program state by market index.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "market_index": { "type": "integer", "description": "The index of the perp market" }
                },
                "required": ["market_index"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Self::fetch_drift_info(args.market_index).await
    }
}

impl DriftInfo {
    pub async fn fetch_drift_info(market_index: u16) -> Result<String, DriftDataError> {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let wallet: Wallet = solana_sdk::signature::Keypair::new().into();

        let client = tokio::task::spawn(async move {
            DriftClient::new(Context::MainNet, rpc_client, wallet).await
        })
        .await
        .map_err(|_| DriftDataError { 
            message: "Failed to init Drift client".to_string()
        })?
        .map_err(|_| DriftDataError { 
            message: "Failed to init Drift client".to_string()
        })?;

        let program_data = client.program_data().clone();

        let market_info = tokio::task::spawn_blocking(move || {
            program_data
                .perp_market_config_by_index(market_index)
                .cloned()
                .unwrap_or_default()
        })
        .await
        .map_err(|_| DriftDataError { 
            message: "Failed to get perp market config by index".to_string()
        })?;

        let state = client.state_account().map_err(|_| DriftDataError { 
            message: "Failed to get client state account".to_string()
        })?;

        Ok(format!(
            "Market Index {} Info: {:?}\nState: {:?}",
            market_index, market_info, state
        ))
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetOpenPositionsArgs {
    pub account_pubkey: Option<String>,
    pub market_index: Option<u16>,
    pub position_type: Option<String>, // "perp", "spot", or "both" (default)
}

pub struct DriftGetOpenPositions;
impl Tool for DriftGetOpenPositions {
    const NAME: &'static str = "get_open_positions";

    type Error = DriftDataError;
    type Args = GetOpenPositionsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_open_positions".to_string(),
            description: "This tool retrieves data about open spot and perp positions and open orders on Drift, and should only be used when the user requests info, data, or a list of positions/orders,—not when they issue a trade command. It can filter by market name or index (e.g., 'DOGE' or 7), account public key, and position type ('long' or 'short', default 'both'), but all inputs are optional.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "account_pubkey": { "type": "string", "description": "Public key of the account (optional, defaults to AGENT_VAULT)" },
                    "market_index": { "type": "integer", "description": "Filter by market index (optional)" },
                    "position_type": { "type": "string", "enum": ["perp", "spot", "both"], "description": "Type of positions to retrieve (default: both)" }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<String, DriftDataError> {
        let result = tokio::task::spawn_blocking(move || {
            let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
            let agent_vault = env::var("AGENT_VAULT").expect("Missing AGENT_VAULT env variable");
            let agent_vault_clone = agent_vault.clone();
            let secret_key_str = env::var("AGENT_KEYPAIR").expect("Missing AGENT_KEYPAIR env variable");
            let secret_key_bytes: Vec<u8> = secret_key_str
                .split(',')
                .map(|s| s.parse::<u8>().expect("Invalid byte"))
                .collect();
    
            let keypair = solana_sdk::signer::keypair::Keypair::from_bytes(&secret_key_bytes)
                .expect("Invalid private key");
            let secret_key_base58 = bs58::encode(keypair.to_bytes()).into_string();
            let mut wallet = Wallet::try_from_str(&secret_key_base58).expect("Failed to load wallet");

            let agent_vault_pubkey = Pubkey::from_str(&agent_vault_clone).unwrap();
            wallet.to_delegated(agent_vault_pubkey);
            let client = tokio::runtime::Handle::current().block_on(async {
                DriftClient::new(
                    Context::MainNet,
                    solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url),
                    wallet.clone(),
                )
                .await
            })
            .map_err(|_| DriftDataError { 
                message: "Failed to init Drift client".to_string()
            }).unwrap();
    
            let open_drift_positions = tokio::runtime::Handle::current().block_on(async {
                Self::get_open_positions(args.clone(), &client).await
            })
            .unwrap();
    
            let result = tokio::runtime::Handle::current().block_on(async {
                Self::format_positions(
                    client,
                    &open_drift_positions.perp_positions,
                    &open_drift_positions.spot_positions,
                    &open_drift_positions.open_orders,
                    args.position_type.unwrap_or("both".to_string()).as_str(),
                )
                .await
            });
    
            result
        })
        .await
        .map_err(|_| DriftDataError { 
            message: "Failed to call get open positions tool".to_string()
        })?;
    
        Ok(result)
    }
    
}


pub struct OpenDriftPositions {
    pub perp_positions: Vec<PerpPosition>,
    pub spot_positions: Vec<SpotPosition>,
    pub open_orders: Vec<Order>,
}

impl DriftGetOpenPositions {
    pub async fn get_open_positions(args: GetOpenPositionsArgs, drift_client: &DriftClient) -> Result<OpenDriftPositions, DriftDataError> {
        let client = drift_client.clone();

        let handle = tokio::task::spawn(async move {
            let user = client.get_user_account(&client.wallet().default_sub_account()).await.map_err(|_| DriftDataError { 
                message: "Failed to get_user_account".to_string()
            })?;
            let filter_index = args.market_index;

            let open_orders: Vec<_> = user.orders
                .iter()
                .filter(|p| p.status == OrderStatus::Open)
                .filter(|p| filter_index.map_or(true, |idx| p.market_index == idx))
                .collect();

            let perp_positions: Vec<_> = user
                .perp_positions
                .iter()
                .filter(|p| p.is_open_position())
                .filter(|p| filter_index.map_or(true, |idx| p.market_index == idx))
                .collect();

            let spot_positions: Vec<_> = user
                .spot_positions
                .iter()
                .filter(|s| !s.is_available())
                .filter(|s| filter_index.map_or(true, |idx| s.market_index == idx))
                .collect();

            let perp_positions_vec: Vec<PerpPosition> = perp_positions.into_iter().cloned().collect();
            let spot_positions_vec: Vec<SpotPosition> = spot_positions.into_iter().cloned().collect();
            let open_orders_vec: Vec<Order> = open_orders.into_iter().cloned().collect();

            let drift_positions = OpenDriftPositions { perp_positions: perp_positions_vec, spot_positions: spot_positions_vec, open_orders: open_orders_vec };
            Ok(drift_positions)
        });

        handle.await.map_err(|_| DriftDataError { 
            message: "Failed to await get_open_positions".to_string()
        })?
    }

    async fn format_perp(drift_client: &DriftClient, parsed_market_data: &serde_json::Value, positions: &[PerpPosition]) -> String {
        if positions.is_empty() {
            "No open perpetual positions.".to_string()
        } else {
            let mut results = Vec::new();
            for p in positions {
                let market_id = MarketId::perp(p.market_index);
                let oracle_price = drift_client.oracle_price(market_id).await.unwrap_or(0);
                let unrealized_pnl = p.get_unrealized_pnl(oracle_price).unwrap_or(0);
    
                results.push(format!(
                    "- Market: {}\n  Base Amount: {}\n  Quote Amount: ${}\n  Unrealized PnL: ${}\n  Entry: ${}\n  Open Orders: {}\n",
                    parsed_market_data["PERP"][p.market_index.to_string()], 
                    p.base_asset_amount as f64 / 1_000_000_000.0, 
                    p.quote_asset_amount as f64 / 1_000_000.0, 
                    unrealized_pnl as f64 / 1_000_000.0,
                    p.quote_entry_amount as f64 / 1_000_000.0,
                    p.open_orders
                ));
            }
            results.join("\n")
        }
    }
    

    pub async fn format_positions(drift_client: DriftClient, perp_positions: &[PerpPosition], spot_positions: &[SpotPosition], open_orders: &[Order], position_type: &str) -> String {
        let parsed_market_data = parse_market_data();

        let perp_str = Self::format_perp(&drift_client, &parsed_market_data, perp_positions).await;
    
        let format_spot = |positions: &[SpotPosition]| -> String {
            if positions.is_empty() {
                "No open spot positions.".to_string()
            } else {
                positions.iter().map(|s| {
                    format!(
                        "- Market: {}\n  Balance: {}\n  Cumulative Deposits: {}\n  Open Orders: {}\n",
                        parsed_market_data["SPOT"][s.market_index.to_string()], 
                        s.scaled_balance as f64 / 1_000_000_000.0, 
                        s.cumulative_deposits as f64 / 1_000_000.0, 
                        s.open_orders
                    )
                }).collect::<Vec<_>>().join("\n")
            }
        };

        let format_orders = |orders: &[Order]| -> String {
            if orders.is_empty() {
                "No open orders.".to_string()
            } else {
                orders.iter().map(|o| {
                    format!(
                        "- Market Type: {}\n Market: {}\n  Order ID: {}\n  Type: {:?}\n  Status: {:?}\n  Price: {}\n  Base Amount: {}\n  Direction: {:?}\n",
                        o.market_type.as_str(), 
                        parsed_market_data[o.market_type.as_str().to_ascii_uppercase()][o.market_index.to_string()], 
                        o.order_id, 
                        o.order_type, 
                        o.status, 
                        o.price as f64 / 1_000_000.0, 
                        o.base_asset_amount as f64 / 1_000_000_000.0, 
                        o.direction
                    )
                }).collect::<Vec<_>>().join("\n")
            }
        };

        match position_type {
            "perp" => format!("**Perp Positions:**{}", perp_str),
            "spot" => format!("**Spot Positions:**\n{}", format_spot(spot_positions)),
            "open_orders" => format!("**Open Orders:**\n{}", format_orders(open_orders)),
            _ => format!(
                "{}\n\n{}\n\n{}",
                format!("**Perp Positions:**\n{}", perp_str),
                format!("**Spot Positions:**\n{}", format_spot(spot_positions)),
                format!("**Open Orders:**\n{}", format_orders(open_orders)),
            ),
        }
    }
}