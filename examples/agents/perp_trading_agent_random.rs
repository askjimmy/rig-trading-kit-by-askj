use std::{env, error::Error, str::FromStr, time::Duration};
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
            for position in open_positions.perp_positions {
                if position.base_asset_amount != 0 {
                    let close_amount = -position.base_asset_amount; 
                    let market_index = position.market_index;

                    let close_order = PerpOrder {
                        market_index,
                        amount: close_amount / 1_000_000_000, 
                        price: None,
                        post_only: Some(false),
                    };

                    println!("Closing open position on market {}: {}", market_index, close_amount);
                    let close_result = DriftPlacePerpOrders::place_perp_orders(DriftPlacePerpOrdersArgs {
                        orders: vec![close_order],
                    }, &client)
                    .await;

                    match close_result {
                        Ok(close_id) => {
                            println!("Closed position successfully: {}", close_id);
                        },
                        Err(e) => println!("Failed to close position on market {}: {:?}", market_index, e),
                    }

                    sleep(Duration::from_secs(5)).await;
                }
            }
        } else {
            println!("Failed to fetch open positions.");
        }


        let mut rng = thread_rng();
        let market_index = rng.gen_range(1..=10);
        let market_id: MarketId = MarketId::new(market_index, MarketType::Perp);
        //let amount = rng.gen_range(1..=5) as i64 * if rng.gen_bool(0.5) { 1 } else { -1 }; // Random long or short
        let price  = get_token_oracle_price(market_id, &client).await.unwrap();
        let price_float = price as f64 / 1_000_000.0; 

        let usdc_value = 50.0;
        let amount = (usdc_value / price_float).round() as i64; 
        if amount == 0 {
            println!("Skipping market {} due to price being too high for a meaningful trade.", market_index);
            continue;
        }

        let order = PerpOrder {
            market_index,
            amount,
            price: None,
            post_only: Some(false),
        };

        println!("Placing order: {:?} on market {}", amount, market_index);

        let order_result = DriftPlacePerpOrders::place_perp_orders(DriftPlacePerpOrdersArgs {
            orders: vec![order],
        }, &client)
        .await;

        match order_result {
            Ok(order_id) => println!("Order placed successfully: {}", order_id),
            Err(e) => {
                println!("Failed to place order: {:?}", e);
                if let Some(source) = e.source() {
                    eprintln!("Caused by: {}", source);
                }
                continue;
            }
        }

        let wait_time = rng.gen_range(20..=22); // Random wait between 5-10 seconds
        println!("Waiting for {} seconds before closing position...", wait_time);
        sleep(Duration::from_secs(wait_time)).await;

        let open_positions = DriftGetOpenPositions::get_open_positions(GetOpenPositionsArgs {
            account_pubkey: None,
            market_index: Some(market_index),
            position_type: None,
        }, &client).await;

        match open_positions {
            Ok(positions) => {
                if !positions.perp_positions.is_empty() {
                    let close_amount = -positions.perp_positions[0].base_asset_amount;
                    let close_order = PerpOrder {
                        market_index,
                        amount: close_amount / 1_000_000_000,
                        price: None,
                        post_only: Some(false),
                    };

                    println!("Closing position on market {}: {}", market_index, close_amount);
                    let close_result = DriftPlacePerpOrders::place_perp_orders(DriftPlacePerpOrdersArgs {
                        orders: vec![close_order],
                    }, &client)
                    .await;

                    match close_result {
                        Ok(close_id) => {
                            println!("Position closed successfully: {}", close_id);
                            let wait_time = rng.gen_range(10..=11);
                            println!("Waiting for {} seconds before opening position...", wait_time);
                            sleep(Duration::from_secs(wait_time)).await;
                        },
                        Err(e) => println!("Failed to close position: {:?}", e),
                    }
                } else {
                    println!("No open position to close for market {}", market_index);
                }
            }
            Err(e) => println!("Failed to fetch open positions: {:?}", e),
        }

        println!("-------------------------------\n");
    }
}
