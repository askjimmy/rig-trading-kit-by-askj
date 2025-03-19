use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::env;
use std::collections::HashMap;
use crate::askj::request::{get_request, post_request};

#[derive(Debug, Serialize, Deserialize)]
struct SampleMessage {
    timestamp: i64,
    message: String,
    #[serde(rename = "publicKey")]
    public_key:String,
    nonce:i64
}

#[derive(Debug, Serialize, Deserialize)]
struct SignMessage {
    timestamp: i64,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginBody {
    #[serde(rename = "publicKey")]
    public_key: String,
    signature: String,
    timestamp:i64,
    nonce:i64
}

pub async fn auth_login(private_key:&str,api_url: &str, oauth_key: &str) -> Result<(String, i64)> {
    //get key pair
    // Check if private_key is an empty string and load from env if necessary
    let private_key = if private_key.is_empty() {
        env::var("AGENT_PRIVATEKEY").context("Missing AGENT_PRIVATEKEY")?
    } else {
        private_key.to_string()
    };
    
    let keypair = Keypair::from_base58_string(&private_key);

    // Get sample message
    let sample_message_url = format!("{}/auth/sample-message?public_key={}", api_url, keypair.pubkey().to_string());
    let query:HashMap<String, String> = HashMap::new();
    let sample_message: SampleMessage = serde_json::from_value(get_request(&sample_message_url, oauth_key,&query).await?)
        .context("Failed to parse sample message")?;

    // Extract timestamp and contents
    let timestamp = sample_message.timestamp;
    let message = sample_message.message.clone();
    let nonce = sample_message.nonce;
    
    // Sign with Solana private key
    let signature = keypair.sign_message(message.as_bytes()).to_string();
    // Prepare the body for the POST request
    let login_body = LoginBody {
        public_key: keypair.pubkey().to_string(),
        signature,
        timestamp,
        nonce
    };
    
    // // Send signed result to auth/login
    let login_url = format!("{}/auth/login", api_url);
    let login_body_json = serde_json::to_value(&login_body).context("Failed to convert login body to JSON")?;
    let response: Value = post_request(&login_url, &login_body_json, oauth_key).await?;
    let token = response["token"].as_str().context("Invalid response from backend.")?;
    let expire = response["expiresInSeconds"].as_i64().context("Invalid response from backend.")?;

    Ok((token.to_string(), expire))
}

