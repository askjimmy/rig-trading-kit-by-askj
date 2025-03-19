use agent_trading_kit::askj::Askj;
use agent_trading_kit::askj::schema::VaultAsignSchema;
use anyhow::Result;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use clap::Parser;

/**
 * Assign a vault to selected agent
 * usage cargo run --example assign_vault --agent-id=[YOUR AGENT ID] [txn_hash] [vault_name] [vault_address]
 */
 
/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    agent_id: String,
    txhash: String,
    vault_name: String,
    vault_address: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    let args = Args::parse();
    let agent_id = args.agent_id.clone();
    let txhash = args.txhash.clone();
    let vault_name = args.vault_name.clone();
    let vault_address = args.vault_address.clone();
    // Initialize the Askj struct
    // Load private key from env variable AGENT_PRIVATEKEY and log in to the ASKJIMMY backend
    // let askj = Askj::new(None).await;
    //
    // If you want to use a different private key, you can pass it in the constructor
    // let askj = Askj::new(some_private_key_string).await;
    let mut askj = Askj::new(None).await;

    match askj.login().await {
        Ok(_) => {
            // Convert the expired timestamp to a human-readable date format
            let expiration_date = Utc
                .timestamp_opt(askj.expired, 0)
                .single()
                .expect("Invalid timestamp")
                .to_rfc3339();
            println!("Login success, will expire at {}", expiration_date);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    //assign vault to selected agent.
    let vault = VaultAsignSchema {
        txhash,
        vault_name,
        vault_address:vault_address.clone(),
        
    };
    let assign_result = askj.assign_vault(&agent_id,&vault).await;
    match assign_result {
        Ok(_) => {
            println!("Assign vault successful, vault address: {}", &vault_address);
            println!("You can deposit USDC to this vault and the agent will trade automatically by following your defined profile");
        }
        Err(err) => {
            eprintln!("Assign vault error: {}", err);
        }
    }

    Ok(())
}
