use std::{env, str::FromStr, time::Duration};
use rand::Rng;
use tokio::time::sleep;
use solana_sdk::pubkey::Pubkey;
use agent_trading_kit::tools::*;
use drift_rs::{types::{Context, MarketId, MarketType}, DriftClient, Wallet};
use rand::thread_rng;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();

    let agent_vault = env::var("AGENT_VAULT").expect("Missing AGENT_VAULT env variable");
    let secret_key_str = env::var("AGENT_KEYPAIR").expect("Missing AGENT_KEYPAIR env variable");
    let secret_key_bytes: Vec<u8> = secret_key_str
        .split(',')
        .map(|s| s.parse::<u8>().expect("Invalid byte"))
        .collect();
    let keypair = solana_sdk::signer::keypair::Keypair::from_bytes(&secret_key_bytes)
        .expect("Invalid private key");
    let secret_key_base58 = bs58::encode(keypair.to_bytes()).into_string();
    let mut wallet = Wallet::try_from_str(&secret_key_base58).expect("Failed to load wallet");
    let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
    wallet.to_delegated(agent_vault_pubkey);

    let client = DriftClient::new(
        Context::MainNet,
        solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url.to_string()),
        wallet.clone(),
    )
    .await
    .unwrap();

    loop {
        println!("Checking for open positions before trading...");
        let open_positions_result = DriftGetOpenPositions::get_open_positions(GetOpenPositionsArgs {
            account_pubkey: None,
            market_index: None,
            position_type: None,
        }, &client).await;

        if let Ok(open_positions) = open_positions_result {
            if !open_positions.perp_positions.is_empty() {
                println!("Waiting for trailing stop orders to close positions...");
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        } else {
            println!("Failed to fetch open positions.");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        let mut rng = thread_rng();
        let market_index = rng.gen_range(1..=10);
        let market_id: MarketId = MarketId::new(market_index, MarketType::Perp);
        
        let price = match get_token_oracle_price(market_id, &client).await {
            Ok(price) => price, // Assign the value if successful
            Err(e) => {
                eprintln!("Error getting token oracle price: {:?}", e);
                //return Err(e.into()); // Handle the error or propagate it
                continue;
            }
        };
        
        let price_float = price as f64 / 1_000_000.0; 

        let usdc_value = 50.0;
        let amount = (usdc_value / price_float).round() as i64; 
        if amount == 0 {
            println!("Skipping market {} due to price being too high for a meaningful trade.", market_index);
            continue;
        }

        let position_type = if rng.gen_bool(0.5) { "long" } else { "short" };
        let trailing_stop_percentage = 2.0; // Example: 2% trailing stop

        println!(
            "Placing {} order on market {} with amount: {}",
            position_type, market_index, amount
        );

        let order_result = DriftTrailingStopOrders::execute_trailing_stop_orders(DriftTrailingStopOrderArgs {
            market_index: Some(market_index),
            total_amount: Some(amount),
            position_type: Some(position_type.to_string()),
            trailing_stop_percentage: Some(trailing_stop_percentage),
            entry_price: None,
        })
        .await;

        match order_result {
            Ok(order_id) => println!("Trailing stop order placed successfully: {}", order_id),
            Err(e) => {
                println!("Failed to place trailing stop order: {:?}", e);
                continue;
            }
        }

        let wait_time = rng.gen_range(20..=25);
        println!("Waiting {} seconds before checking positions again...", wait_time);
        sleep(Duration::from_secs(wait_time)).await;
    }
}
