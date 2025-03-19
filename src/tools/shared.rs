
use std::time::Duration;

use drift_rs::types::accounts::User;
use drift_rs::types::{MarketId, OrderParams};
use drift_rs::{DriftClient, Wallet};
use tokio::time::sleep;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct DriftDataError {
    pub message: String,
}


pub const SOLANA_MAINNET_RPC_URL: &str = "https://lb.drpc.org/ogrpc?network=solana&dkey=AmRA_MBTVUg_k7dH7a0T0AZ6IfN-An0R8IDpfhHoK236";//
//pub const SOLANA_MAINNET_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

const MAX_RETRIES: usize = 5;
const RETRY_DELAY_MS: u64 = 1000;

pub async fn get_token_oracle_price(market_id: MarketId, client: &DriftClient) -> Result<i64, DriftDataError> {
    let mut attempt = 0;
    
    while attempt <= MAX_RETRIES {
        match client.oracle_price(market_id).await {
            Ok(price) => return Ok(price),
            Err(_err) => {
                //eprintln!("Failed to fetch oracle price: {:?}. Retrying...", err);
                attempt += 1;
                let delay = RETRY_DELAY_MS * (2_u64.pow(attempt as u32));
                sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    eprintln!("Max retries reached for oracle price. Skipping order.");
    Err(DriftDataError{message:"Max retries reached".to_string()})
}

pub async fn send_order_with_retry(
    client: &DriftClient,
    wallet: &Wallet,
    user: &User,
    new_order: OrderParams,
) -> Result<solana_sdk::signature::Signature, DriftDataError> {
    let mut attempt = 0;

    loop {
        if attempt > MAX_RETRIES {
            eprintln!("Max retries reached for sending order. Skipping.");
            return Err(DriftDataError{ message: "Max retries reached for sending order".to_string()});
        }

        match client.sign_and_send(
            drift_rs::TransactionBuilder::new(
                client.program_data(),
                wallet.default_sub_account(),
                std::borrow::Cow::Borrowed(user),
                true,
            )
            .place_orders(vec![new_order])
            .build(),
        )
        .await
        {
            Ok(tx_signature) => return Ok(tx_signature),
            Err(_err) => {
                attempt += 1;
                let delay = RETRY_DELAY_MS * (2_u64.pow(attempt as u32));
                sleep(Duration::from_millis(delay)).await;
            }
        }
    }
}