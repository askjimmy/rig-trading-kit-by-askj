use std::{env, str::FromStr};

use solana_sdk::pubkey::Pubkey;
use agent_trading_kit::tools::*;
use drift_rs::{types::Context, DriftClient, Wallet};
use tokio;

#[tokio::main]
async fn main() {
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

        let client = DriftClient::new(
            Context::MainNet,
            solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url.to_string()),
            wallet.clone(),
        ).await.unwrap();

    // // Deposit
    // let deposit_result = Deposit::get_tx_data(DepositArgs {
    //     amount: 10_000,
    //     spot_market_index: 1,
    //     user_token_account: "EXAMPLE_PUBLIC_KEY".to_string(),
    //     reduce_only: Some(false),
    // }).await;

    // match deposit_result {
    //     Ok(tx) => println!("Deposit transaction data: {}", tx),
    //     Err(_) => println!("Deposit failed!"),
    // }

    // // Withdrawal
    // let withdraw_result = Withdraw::get_tx_data(WithdrawArgs {
    //     amount: 5_000, 
    //     spot_market_index: 1, 
    //     user_token_account: "EXAMPLE_PUBLIC_KEY".to_string(),
    //     reduce_only: Some(false),
    // }).await;

    // match withdraw_result {
    //     Ok(tx) => println!("Withdraw transaction data: {}", tx),
    //     Err(_) => println!("Withdrawal failed!"),
    // }

    // // Get Drift Info for market
    // let market_index = 6; // ARB-PERP
    // let drift_market_info = DriftInfo::fetch_drift_info(market_index).await;

    // match drift_market_info {
    //     Ok(market_info) => println!("Market info for market index {}: {}", market_index, market_info),
    //     Err(_) => println!("Query failed!"),
    // }

    // // Place perp order on Drift
    // let perp_response = DriftPlaceComplexPerpOrders::place_complex_perp_orders(DriftPlaceComplexPerpOrdersArgs{
    //     orders: vec![
    //         PerpOrder {
    //             market_index: 7,   // DOGE market index
    //             price: None,
    //             amount: 50,
    //             post_only: Some(false), // Explicitly set post_only
    //         },
    //         PerpOrder {
    //             market_index: 6,   // ARB market index
    //             amount: -5,        // Short 5
    //             price: None,
    //             post_only: Some(false), // Explicitly set post_only
    //         },
    //     ],
    // }
    // ).await;
        
    // match perp_response {
    //     Ok(perp_id) => println!("Perp response: {}", perp_id),
    //     Err(_) => println!("Order failed!"),
    // }

    // Get info on open positions and orders
    let open_positions = DriftGetOpenPositions::get_open_positions(GetOpenPositionsArgs {
        account_pubkey: None, // Optional (Defaults to agent vault from env)
        market_index: None, // optional filter
        position_type: None // optional filter
        
    }, &client).await;

    match open_positions {
        Ok(open_positions_data) => println!("Perp positions: {:?}, Spot positions: {:?}, Open orders: {:?}", open_positions_data.perp_positions, open_positions_data.spot_positions, open_positions_data.open_orders),
        Err( drift_data_error ) => println!("Failed to query info on open positions: {}", drift_data_error),
    }


    // // Get info from Drift vault
    // let drift_vault_params = DriftVaultInfoArgs {
    //     vault_address: None, // optional, defaults to Agent vault
    //     requested_fields: None, // Optional, defaults to all
    // };

    // let drift_vault_response = DriftVaultInfo::fetch_vault_info(drift_vault_params).await;
    // match drift_vault_response {
    //     Ok(drift_vault_data) => println!("Vault data {}", drift_vault_data),
    //     Err( drift_data_error ) => println!("Failed to query drift vault data: {}", drift_data_error),
    // }

    

}
