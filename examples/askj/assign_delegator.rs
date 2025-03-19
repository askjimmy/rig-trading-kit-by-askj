use agent_trading_kit::askj::Askj;
use anyhow::Result;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use clap::Parser;

/**
 * Assign a delegator to selected agent
 * usage cargo run --example assign_delegator --agent-id=[YOUR AGENT ID]
 */
 
/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    agent_id: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    let args = Args::parse();
    let agent_id = args.agent_id.clone();
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

    //assign delegator, delegator will be generated and assign on the agent
    let assign_result = askj.assign_delegator(&agent_id).await;
    match assign_result {
        Ok(delegator_address) => {
            println!("Assign delegator successful, delegator address: {}", delegator_address);
            println!("The agent will trade by using this delegator, You need to keep Sol on this delegator");
        }
        Err(err) => {
            eprintln!("Assign delegator error: {}", err);
        }
    }

    Ok(())
}
