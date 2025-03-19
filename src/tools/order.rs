use std::{collections::HashMap, sync::Arc, time::Duration};
use std::{env, str::FromStr, sync::OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tokio::{sync::Mutex, time::sleep};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;

use solana_sdk::{native_token::LAMPORTS_PER_SOL, signature::Keypair};
use solana_client::nonblocking::rpc_client::RpcClient;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use drift_rs::{math::constants::{LAMPORTS_PER_SOL_I64, PRICE_PRECISION_U64}, types::{Context, MarketId, MarketType, OrderParams}, DriftClient, Pubkey, Wallet};

use crate::tools::shared::*;

#[derive(Deserialize, Serialize)]
pub struct PerpOrder {
    pub market_index: u16,
    pub amount: i64,
    pub price: Option<u64>, // Optional price
    pub post_only: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct DriftPlacePerpOrdersArgs {
    pub orders: Vec<PerpOrder>, // Accept multiple orders
}

#[derive(Deserialize, Serialize)]
pub struct DriftPlacePerpOrders;

impl Tool for DriftPlacePerpOrders {
    const NAME: &'static str = "drift_place_perp_orders";

    type Error = DriftDataError;
    type Args = DriftPlacePerpOrdersArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_place_perp_orders".to_string(),
            description: "Places multiple perp orders on Drift in a single transaction using drift-rs.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "orders": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "market_index": { "type": "integer", "description": "The market index for the perp trade." },
                                "amount": { "type": "integer", "description": "The order amount in base asset units (signed for long/short positions)." },
                                "price": { "type": "integer", "description": "The order price in Drift price precision. If omitted, it is automatically calculated." },
                                "post_only": { "type": "boolean", "description": "Whether the order should be post-only." }
                            },
                            "required": ["market_index", "amount"]
                        }
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        let rpc_url = SOLANA_MAINNET_RPC_URL;
        let agent_vault: String = env::var("AGENT_VAULT").unwrap();
        let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
        let secret_key_bytes: Vec<u8> = secret_key_str
            .split(',')
            .map(|s| s.parse::<u8>().expect("Invalid byte"))
            .collect();
    
        let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");
        let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
        let mut wallet: Wallet = Wallet::from(keypair);
        wallet.to_delegated(agent_vault_pubkey);

        let handle = tokio::task::spawn(async move {
            let client = DriftClient::new(
                Context::MainNet,
                RpcClient::new(rpc_url.to_string()),
                wallet.clone(),
            )
            .await
            .map_err(|_| DriftDataError { 
                message: "Failed to init Drift client".to_string(),
            })?;
        
            Self::place_perp_orders(args, &client).await
        });
        handle.await.map_err(|_| DriftDataError { 
            message: "Failed to await place_perp_orders".to_string()
        })?
    }
    
    // async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
    //     let rpc_url = SOLANA_MAINNET_RPC_URL;
    //     let agent_vault: String = env::var("AGENT_VAULT").unwrap();
    //     let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
    //     let secret_key_bytes: Vec<u8> = secret_key_str
    //         .split(',')
    //         .map(|s| s.parse::<u8>().expect("Invalid byte"))
    //         .collect();

    //     let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");
    //     let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
    //     let mut wallet: Wallet = Wallet::from(keypair);
    //     wallet.to_delegated(agent_vault_pubkey);

    //     let client = DriftClient::new(
    //         Context::MainNet,
    //         RpcClient::new(rpc_url.to_string()),
    //         wallet.clone(),
    //     )
    //     .await
    //     .map_err(|_| DriftDataError { 
    //         message: "Failed to init Drift client".to_string()
    //     })?;

    //     tokio::task::spawn_blocking(move || {
    //         tokio::runtime::Runtime::new()
    //             .unwrap()
    //             .block_on(Self::place_perp_orders(args, &client))
    //     })
    //     .await
    //     .unwrap()
    // }
}

impl DriftPlacePerpOrders {
    pub async fn place_perp_orders(
        args: DriftPlacePerpOrdersArgs, drift_client: &DriftClient,
    ) -> Result<String, DriftDataError> {
        // let rpc_url = SOLANA_MAINNET_RPC_URL;
        // let agent_vault: String = env::var("AGENT_VAULT").unwrap();
        // let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
        // let secret_key_bytes: Vec<u8> = secret_key_str
        //     .split(',')
        //     .map(|s| s.parse::<u8>().expect("Invalid byte"))
        //     .collect();

        // let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");
        // let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
        // let mut wallet: Wallet = Wallet::from(keypair);
        // wallet.to_delegated(agent_vault_pubkey);

        // let client = DriftClient::new(
        //     Context::MainNet,
        //     RpcClient::new(rpc_url.to_string()),
        //     wallet.clone(),
        // )
        // .await
        // .map_err(|_| DriftDataError { 
        //     message: "Failed to init Drift client".to_string()
        // })?;
        let client = drift_client.clone();
        let user = client
            .get_user_account(&drift_client.wallet().default_sub_account())
            .await
            .map_err(|_| DriftDataError { 
                message: "Failed to init Drift user".to_string()
            })?;

        let mut order_requests = Vec::new();

        for order in args.orders {
            let market_id = MarketId::perp(order.market_index);

            let price = order.price.unwrap_or(0);
            if price > 0 {
                order_requests.push(
                    drift_rs::types::NewOrder::limit(market_id)
                        .amount(order.amount * LAMPORTS_PER_SOL_I64)
                        .price(price * PRICE_PRECISION_U64)
                        .post_only(if order.post_only.unwrap_or(false) {
                            drift_rs::types::PostOnlyParam::MustPostOnly
                        } else {
                            drift_rs::types::PostOnlyParam::None
                        })
                        .build(),
                );
            } else {
                order_requests.push(
                    drift_rs::types::NewOrder::market(market_id)
                        .amount(order.amount * LAMPORTS_PER_SOL_I64)
                        .post_only(if order.post_only.unwrap_or(false) {
                            drift_rs::types::PostOnlyParam::MustPostOnly
                        } else {
                            drift_rs::types::PostOnlyParam::None
                        })
                        .build(),
                );
            };
        }

        let order_request = order_requests.first().unwrap();
        let order_response = send_order_with_retry(&client, &client.wallet(), &user, *order_request).await;

        match order_response {
            Ok(tx_signature) => {
                client.unsubscribe().await.map_err(|_| DriftDataError { 
                    message: "Failed to unsubscribe".to_string()
                })?;
                Ok(format!("Perp order(s) placed successfully: {:?}", tx_signature))
            }
            Err(err) => {
                let err_msg = err.to_string();
                if err_msg.contains("Blockhash not found") {
                    eprintln!("Blockhash expired, returning error instead of quitting.");
                    return Err(DriftDataError { message: "Blockhash expired".to_string() });
                }
                Err(DriftDataError { message: err_msg })
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TWAPOrder {
    pub market_index: u16,
    pub total_amount: i64,
    pub total_duration_secs: u64,
    pub interval_secs: u64,
    pub order_type: String,
}

#[derive(Default)]
struct TWAPTracker {
    orders: Mutex<HashMap<String, u64>>,
}

#[derive(Deserialize, Serialize)]
pub struct DriftTWAPOrdersArgs {
    pub twap_orders: Vec<TWAPOrder>,
}

#[derive(Deserialize, Serialize)]
pub struct DriftTWAPOrders;

impl Tool for DriftTWAPOrders {
    const NAME: &'static str = "drift_twap_orders";

    type Error = DriftDataError;
    type Args = DriftTWAPOrdersArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_twap_orders".to_string(),
            description: "Executes TWAP orders over time on Drift using drift-rs.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "twap_orders": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "market_index": { "type": "integer", "description": "The market index for the perp trade." },
                                "total_amount": { "type": "integer", "description": "The total amount of base asset to trade." },
                                "total_duration_secs": { "type": "integer", "description": "The duration over which to execute the TWAP." },
                                "interval_secs": { "type": "integer", "description": "Interval between each trade execution in seconds." },
                                "order_type": { "type": "string", "enum": ["market", "limit"], "description": "Order type." }
                            },
                            "required": ["market_index", "total_amount", "total_duration_secs", "interval_secs", "order_type"]
                        }
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        Self::execute_twap_orders(args).await
    }
}

impl DriftTWAPOrders {
    pub async fn execute_twap_orders(args: DriftTWAPOrdersArgs) -> Result<String, DriftDataError> {
        static TWAP_TRACKER: OnceLock<Arc<TWAPTracker>> = OnceLock::new();
        let twap_tracker = TWAP_TRACKER.get_or_init(|| Arc::new(TWAPTracker::default())).clone();
        let order_id = Uuid::new_v4().to_string();

        let tracker = twap_tracker.orders.lock().await;
        if !tracker.is_empty() {
            return Ok(format!("Current TWAP progress: {:?}", *tracker));
        }
        drop(tracker);

        let twap_tracker_clone = Arc::clone(&twap_tracker);
        let order_id_clone = order_id.clone();

        tokio::spawn(async move {
            let rpc_url = SOLANA_MAINNET_RPC_URL;
            let agent_vault: String = env::var("AGENT_VAULT").unwrap();
            let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
            let secret_key_bytes: Vec<u8> = secret_key_str.split(',').map(|s| s.parse::<u8>().expect("Invalid byte")).collect();
            let kp = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");
            let mut wallet = Wallet::from(kp);
            let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
            wallet.to_delegated(agent_vault_pubkey);
            let wallet_clone = wallet.clone();
            let client = DriftClient::new(
                Context::MainNet,
                RpcClient::new(rpc_url.to_string()),
                wallet_clone,
            ).await.unwrap();

            let user = client.get_user_account(&wallet.default_sub_account()).await.unwrap();

            for order in args.twap_orders {
                let market_id = MarketId::perp(order.market_index);
                let num_trades = order.total_duration_secs / order.interval_secs;
                let trade_size = order.total_amount / num_trades as i64;
                for _ in 0..num_trades {
                    let price  = get_token_oracle_price(market_id, &client).await.unwrap();
                    let order_price = Some((price as f64 * 1.005) as u64 * PRICE_PRECISION_U64);
                    let new_order: OrderParams;
                    if order.order_type == "limit" {
                        new_order = drift_rs::types::NewOrder::limit(market_id)
                            .amount(trade_size * LAMPORTS_PER_SOL_I64)
                            .price(order_price.unwrap_or_default())
                            .build();
                    } else {
                        new_order = drift_rs::types::NewOrder::market(market_id)
                            .amount(trade_size * LAMPORTS_PER_SOL_I64)
                            .build();
                    }

                    if let Err(_err) = send_order_with_retry(&client, &wallet, &user, new_order).await {
                        //eprintln!("Final failure: {:?}", err);
                        continue;
                    }

                    let mut tracker = twap_tracker_clone.orders.lock().await;
                    *tracker.entry(order_id_clone.clone()).or_insert(0) += 1;
                    drop(tracker);
                    sleep(Duration::from_secs(order.interval_secs)).await;
                }
            }
        });
        Ok(format!("TWAP orders initiated with ID: {}", order_id))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DriftVWAPOrdersArgs {
    pub market_index: Option<i32>,
    pub size_per_order: Option<i64>,
    pub timeframe: Option<i32>,
    pub history_warm_up: Option<i32>,
    pub stop_signal: Option<bool>,
    pub duration_secs: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeData {
    pub timestamp: i64,
    pub price: f64,
    pub volume: f64,
}

#[derive(Default)]
pub struct VWAPTracker {
    pub trades: Mutex<Vec<TradeData>>,          // List of historical trades for VWAP calculation
    pub vwap: Mutex<f64>,                       // Current calculated VWAP value
    pub history_warm_up: Mutex<i32>,            // Number of intervals to skip for warm-up
    pub stop_signal: Mutex<Option<bool>>,       // Flag to stop the strategy
    pub last_trade_time: Mutex<i64>,        // Last time a trade was placed
    pub history_ready: Mutex<bool>,             // Flag to indicate if enough history is available for calculation
    pub orders: Mutex<HashMap<String, usize>>,
}

pub async fn calculate_vwap(tracker: &VWAPTracker, price: f64) -> Result<f64, DriftDataError> {
    let trades = tracker.trades.lock().await;

    if trades.is_empty(){
        return Ok(price);
    }

    let total_volume = trades.iter().map(|trade| trade.volume).sum::<f64>();
    let vwap = trades.iter().map(|trade| trade.price * trade.volume).sum::<f64>() / total_volume;

    Ok(vwap)
}


#[derive(Deserialize, Serialize)]
pub struct DriftVWAPOrders;
impl DriftVWAPOrders{
    pub fn stop_vwap(&self, stop_signal: Arc<AtomicBool>) {
        stop_signal.store(true, Ordering::SeqCst);
    }
}

impl Tool for DriftVWAPOrders {
    const NAME: &'static str = "drift_vwap_orders";

    type Error = DriftDataError;
    type Args = DriftVWAPOrdersArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_vwap_orders".to_string(),
            description: "Executes VWAP orders on Drift perps, with an optional warm-up period and run-time duration. Can also return info on existing VWAP orders".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "market_index": { "type": "integer", "description": "The market index for the perp trade." },
                    "size_per_order": { "type": "integer", "description": "The size of each order to place." },
                    "timeframe": { "type": "integer", "description": "The interval in seconds to recalculate VWAP. (default is 10 seconds)" },
                    "history_warm_up": { "type": "integer", "description": "The number of intervals to skip for warm-up (default is 5)." },
                    "stop_signal": { "type": "boolean", "description": "A flag to stop the VWAP strategy." },
                    "duration_secs": { "type": "integer", "description": "The duration in seconds for how long to run the VWAP strategy (optional). If not provided, runs indefinitely." }
                },
                "required": ["market_index", "size_per_order", "timeframe"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        static VWAP_TRACKER: OnceLock<Arc<VWAPTracker>> = OnceLock::new();
        let vwap_tracker = VWAP_TRACKER.get_or_init(|| Arc::new(VWAPTracker::default())).clone();

        let is_query_request = args.history_warm_up.is_none()
        && args.stop_signal.is_none()
        && args.duration_secs.is_none()
        && args.size_per_order.unwrap_or(0) == 0
        && args.timeframe.unwrap_or(0) == 0;

        println!("Is query request: {}", is_query_request);
        println!("history_warm_up: {:?}", args.history_warm_up);
        println!("stop_signal: {}", args.stop_signal.is_none());
        println!("duration_secs: {}", args.duration_secs.is_none());
        println!("size_per_order: {}", args.size_per_order.unwrap_or(0));
        println!("timeframe: {}", args.timeframe.unwrap_or(0));

        if is_query_request {
            let trades_count = vwap_tracker.trades.lock().await.len();
            let vwap = *vwap_tracker.vwap.lock().await;
            let history_ready = *vwap_tracker.history_ready.lock().await;
            let orders = vwap_tracker.orders.lock().await.clone();

            return Ok(format!(
                "VWAP Tracker Info:\n- Current VWAP: {:.6}\n- Trade History Size: {}\n- History Ready: {}\n- Active Orders: {:?}",
                vwap, trades_count, history_ready, orders
            ));
        }

        let history_warm_up = args.history_warm_up.unwrap_or(5);
        let market_index = args.market_index;
        let size_per_order = args.size_per_order;
        let recalculation_interval = args.timeframe;

        let stop_signal = Arc::new(AtomicBool::new(false));

        let duration_secs = args.duration_secs.unwrap_or(0);
        let stop_time = if duration_secs > 0 {
            Some(Instant::now() + Duration::from_secs(duration_secs as u64))
        } else {
            None
        };

        let order_id = uuid::Uuid::new_v4().to_string();

        let vwap_tracker_clone = Arc::clone(&vwap_tracker);
        let order_id_clone = order_id.clone();

        tokio::spawn(async move {
            let rpc_url = SOLANA_MAINNET_RPC_URL;
            let agent_vault: String = env::var("AGENT_VAULT").unwrap();
            let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
            let secret_key_bytes: Vec<u8> = secret_key_str
                .split(',')
                .map(|s| s.parse::<u8>().expect("Invalid byte"))
                .collect();
            let kp = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");

            let mut wallet: Wallet = Wallet::from(kp);

            let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
            wallet.to_delegated(agent_vault_pubkey);
            let wallet_clone = wallet.clone(); 
            let client = DriftClient::new(
                Context::MainNet,
                RpcClient::new(rpc_url.to_string()),
                wallet_clone,
            ).await.unwrap();

            let user = client
                .get_user_account(&wallet.default_sub_account())
                .await
                .map_err(|_| DriftDataError { 
                    message: "Failed to init Drift user".to_string()
                }).unwrap();

            let mut interval_counter = 0;

            loop {
                if stop_signal.load(Ordering::SeqCst) || stop_time.map_or(false, |t| Instant::now() > t) {
                    println!("VWAP strategy stopped.");
                    break;
                }

                if interval_counter < history_warm_up {
                    interval_counter += 1;
                    sleep(Duration::from_secs(recalculation_interval.unwrap_or(0) as u64)).await;
                    continue;
                }

                let market_id = MarketId::from((market_index.unwrap_or(0) as u16, MarketType::Perp)); 
                let price = client.oracle_price(market_id).await.unwrap();
                let price_scaled = (price as f64) / 1_000_000.0;
                let vwap_price = calculate_vwap(&vwap_tracker, price_scaled).await.unwrap();

                

                let order_price = (vwap_price * PRICE_PRECISION_U64 as f64) as u64; 

                let order = drift_rs::types::NewOrder::limit(market_id)
                    .amount(size_per_order.unwrap_or(0) * LAMPORTS_PER_SOL_I64)
                    .price(order_price)
                    .build();

                client
                    .sign_and_send(drift_rs::TransactionBuilder::new(
                        client.program_data(),
                        wallet.default_sub_account(),
                        std::borrow::Cow::Borrowed(&user),
                        true,
                    ).place_orders(vec![order]).build()).await.unwrap();

                let mut tracker = vwap_tracker_clone.orders.lock().await;
                tracker.entry(order_id_clone.clone()).or_insert(0);
                drop(tracker);

                sleep(Duration::from_secs(recalculation_interval.unwrap_or(0) as u64)).await;
            }
        });

        Ok(format!("VWAP orders initiated with ID: {}", order_id))
    }

}


#[derive(Default)]
struct TrailingStopTracker {
    orders: Mutex<HashMap<String, f64>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DriftTrailingStopOrderArgs {
    pub market_index: Option<u16>,
    pub position_type: Option<String>,
    pub entry_price: Option<f64>,
    pub trailing_stop_percentage: Option<f64>, 
    pub total_amount: Option<i64>
}
#[derive(Deserialize, Serialize)]
pub struct DriftTrailingStopOrders;

impl Tool for DriftTrailingStopOrders {
    const NAME: &'static str = "drift_trailing_stop_orders";

    type Error = DriftDataError;
    type Args = DriftTrailingStopOrderArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "drift_trailing_stop_orders".to_string(),
            description: "Executes trailing stop orders on Drift and returns information on existing/previous trailing stop orders.".to_string(),
            parameters: json!({
                "type": "object",
                    "properties": {
                        "market_index": { "type": "integer", "description": "The market index for the perp trade." },
                        "position_type": { "type": "string", "enum": ["long", "short"], "description": "Position type." },
                        "trailing_stop_percentage": { "type": "number", "description": "The trailing stop percentage." },
                        "total_amount": { "type": "integer", "description": "The total amount of base asset to trade." },
                        "entry_price": { "type": "number", "description": "The price for limit orders (optional, if not provided, a market order is used)." }
                    },
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        Self::execute_trailing_stop_orders(args).await
    }
}

impl DriftTrailingStopOrders {
    pub async fn execute_trailing_stop_orders(args: DriftTrailingStopOrderArgs) -> Result<String, DriftDataError> {
        static TRAILING_STOP_TRACKER: OnceLock<Arc<TrailingStopTracker>> = OnceLock::new();
        let trailing_stop_tracker = TRAILING_STOP_TRACKER.get_or_init(|| Arc::new(TrailingStopTracker::default())).clone();
    
        let order_id = uuid::Uuid::new_v4().to_string();
    
        let tracker = trailing_stop_tracker.orders.lock().await;
        if !tracker.is_empty() {
            return Ok(format!("Current trailing stop progress: {:?}", *tracker));
        }
        drop(tracker);
    
        let trailing_stop_tracker_clone = Arc::clone(&trailing_stop_tracker);
        let order_id_clone = order_id.clone();

        tokio::spawn(async move {
            let rpc_url = SOLANA_MAINNET_RPC_URL;
            let agent_vault: String = env::var("AGENT_VAULT").unwrap();
            let secret_key_str = env::var("AGENT_KEYPAIR").unwrap();
            let secret_key_bytes: Vec<u8> = secret_key_str
                .split(',')
                .map(|s| s.parse::<u8>().expect("Invalid byte"))
                .collect();
            let kp = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");
    
            let mut wallet: Wallet = Wallet::from(kp);
            let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
            wallet.to_delegated(agent_vault_pubkey);
            let wallet_clone = wallet.clone();
            let client = DriftClient::new(
                Context::MainNet,
                RpcClient::new(rpc_url.to_string()),
                wallet_clone,
            ).await.unwrap();
    
            let user = client
                .get_user_account(&wallet.default_sub_account())
                .await
                .map_err(|_| DriftDataError { 
                    message: "Failed to init Drift user".to_string()
                }).unwrap();
    
            let market_id = MarketId::perp(args.market_index.unwrap());
            let entry_price  = get_token_oracle_price(market_id, &client).await.unwrap() as f64 / LAMPORTS_PER_SOL as f64;
            //let entry_price = client.oracle_price(market_id).await.unwrap() as f64 / LAMPORTS_PER_SOL as f64;
            let trailing_stop_percentage = args.trailing_stop_percentage.unwrap_or(5.0) / 100.0;
            let new_order = drift_rs::types::NewOrder::market(market_id)
                .amount(args.total_amount.unwrap() * LAMPORTS_PER_SOL_I64)
                //.price(entry_price_scaled) // TODO: limit orders
                .build();
            //client.sign_and_send(drift_rs::TransactionBuilder::new(
            //    client.program_data(),
            //    wallet.default_sub_account(),
            //    std::borrow::Cow::Borrowed(&user),
            //    true,
            //).place_orders(vec![new_order]).build()).await.unwrap();

            if let Err(err) = send_order_with_retry(&client, &wallet, &user, new_order).await {
                eprintln!("Failed to send open Trailing Stop order: {:?}", err);
                return;
            }

            let mut highest_price = entry_price;
            let mut lowest_price = entry_price;
            let mut stop_price = entry_price * (1.0 - trailing_stop_percentage / 100.0);
    
            loop {
                let current_price  = get_token_oracle_price(market_id, &client).await.unwrap() as f64 / LAMPORTS_PER_SOL as f64;
                //let current_price = client.oracle_price(market_id).await.unwrap() as f64 / LAMPORTS_PER_SOL as f64;
            
                if let Some(position_type) = &args.position_type {
                    if (position_type == "long" && current_price < stop_price) ||
                       (position_type == "short" && current_price > stop_price) {
                        println!("Trailing stop price hit: {}. Closing position", stop_price);
                        let close_order = drift_rs::types::NewOrder::market(market_id)
                            .amount(if position_type == "long" {
                                -args.total_amount.unwrap() * LAMPORTS_PER_SOL_I64
                            } else {
                                args.total_amount.unwrap() * LAMPORTS_PER_SOL_I64
                            })
                            .build();
                        client.sign_and_send(drift_rs::TransactionBuilder::new(
                            client.program_data(),
                            wallet.default_sub_account(),
                            std::borrow::Cow::Borrowed(&user),
                            true,
                        ).place_orders(vec![close_order]).build()).await.unwrap();
                        break;
                    }
                    if position_type == "long" && current_price > highest_price {
                        highest_price = current_price;
                        stop_price = highest_price * (1.0 - trailing_stop_percentage / 100.0);
                    } else if position_type == "short" && current_price < lowest_price {
                        lowest_price = current_price;
                        stop_price = lowest_price * (1.0 + trailing_stop_percentage / 100.0);
                    }
                } else {
                    break;
                }
                let mut tracker = trailing_stop_tracker_clone.orders.lock().await;
                *tracker.entry(order_id_clone.clone()).or_insert(current_price) = stop_price;
                drop(tracker);
                sleep(Duration::from_secs(3)).await;
            }
        });
    
        Ok(format!("Trailing stop orders initiated with ID: {}", order_id))
    }
}


#[derive(Deserialize, Serialize)]
pub struct DriftClosePerpPositionArgs {
    pub market_index: u16,     // The market index of the asset
    pub position_type: Option<String>, // "long" or "short"
    pub percentage: Option<f64>,       // 0.0 to 1.0 (e.g., 1.0 = 100%, 0.5 = 50%)
}

#[derive(Deserialize, Serialize)]
pub struct DriftClosePerpPosition;

impl Tool for DriftClosePerpPosition {
    const NAME: &'static str = "close_perp_position";

    type Error = DriftDataError;
    type Args = DriftClosePerpPositionArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "close_perp_position".to_string(),
            description: "This tool closes an open position in a specified Drift market when the user explicitly requests to close, exit, reduce, sell, or liquidate a position. The user must specify a market name or index (e.g., 'DOGE' or 7), and optionally a percentage to close (0-100, default 100%) and position type ('long' or 'short').".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "market_index": { "type": "integer", "description": "The market index for the perp trade." },
                    "position_type": { "type": "string", "enum": ["long", "short"], "description": "The position type to close." },
                    "percentage": { "type": "number", "minimum": 0.01, "maximum": 1.0, "description": "Percentage of the position to close (1.0 = 100%)." }
                },
                "required": ["market_index"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, DriftDataError> {
        tokio::spawn(async move {
            Self::execute_close_perp_position(args).await
        })
        .await
        .map_err(|_| DriftDataError { 
            message: "Failed to execute close_perp_position".to_string()
        })?
    }
}

impl DriftClosePerpPosition {
    async fn execute_close_perp_position(args: DriftClosePerpPositionArgs) -> Result<String, DriftDataError> {
        let rpc_url = SOLANA_MAINNET_RPC_URL.to_string();
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

        let client_result = DriftClient::new(
            Context::MainNet,
            solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url.to_string()),
            wallet.clone(),
        )
        .await;
        let client = match client_result {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Error initializing DriftClient: {:?}", e);
                return Ok(format!("Error: Failed to initialize DriftClient: {:?}", e)); // Prevent quitting
            }
        };
        let open_positions_result = crate::tools::DriftGetOpenPositions::get_open_positions(super::GetOpenPositionsArgs {
            account_pubkey: None, // Defaults to agent vault
            market_index: Some(args.market_index), // Filter by market index
            position_type: args.position_type.clone(), // Filter by long/short
        }, &client)
        .await;
        //.unwrap();

        let open_positions = match open_positions_result {
            Ok(positions) => positions,
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("429 Too Many Requests") {
                    eprintln!("Rate limit exceeded, returning error instead of quitting.");
                    return Err(DriftDataError { 
                        message: "Failed to get open positions".to_string()
                    });
                }
                return Err(DriftDataError { 
                    message: format!("DriftDataError in DriftGetOpenPositions: {}", err_msg),
                });
            }
        };

        let pos_type_filter = args.position_type.unwrap_or("both".to_string());
        let positions: Vec<_> = open_positions.perp_positions
            .into_iter()
            .filter(|pos| pos.market_index == args.market_index && 
                (pos_type_filter == "both" || 
                (pos_type_filter == "long" && pos.base_asset_amount > 0) || 
                (pos_type_filter == "short" && pos.base_asset_amount < 0))
            )  
            .collect();

        if positions.is_empty() {
            return Ok(format!(
                "No open {} position found for market index {}",
                pos_type_filter, args.market_index
            ));
        }

        if positions.len() > 1 {
            return Ok(format!(
                "Multiple {} positions found for market index {}. Please specify which to close.",
                pos_type_filter, args.market_index
            ));
        }

        let position = &positions[0];
        let close_percent = args.percentage.unwrap_or(1.0);

        let close_amount = ((position.base_asset_amount as f64) * close_percent).round() as i64;
        if close_amount == 0 {
            return Ok("Close amount is too small.".to_string());
        }
        let close_order = PerpOrder {
            market_index: args.market_index,
            amount: -close_amount / LAMPORTS_PER_SOL_I64, // Negative to close the position
            price: None,
            post_only: None,
        };
        // let close_result = DriftPlacePerpOrders::place_perp_orders(DriftPlacePerpOrdersArgs {
        //     orders: vec![close_order],
        // })
        // .await;

        // Ok(format!("Position closed successfully: {}", close_result.unwrap()))
        match DriftPlacePerpOrders::place_perp_orders(DriftPlacePerpOrdersArgs {
            orders: vec![close_order],
        }, &client)
        .await {
            Ok(result) => Ok(format!("Position closed successfully: {}", result)),
            Err(e) => {
                eprintln!("Error closing position: {:?}", e);  // Log the error
                return Ok(format!("Error: Failed to close position: {:?}", e));
                //Err(DriftDataError)  // Return error instead of panicking
            }
        }
    }
}
