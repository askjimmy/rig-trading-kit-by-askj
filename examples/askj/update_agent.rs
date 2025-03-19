mod agent_profile;

use agent_trading_kit::askj::{schema::{AgentDetailSchema, AgentProfileSchema}, Askj};
use anyhow::Result;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use clap::Parser;

/**
 * Update agent
 * Usage cargo run --example update_agent --agent-id=[YOUR AGENT ID]
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

    //get profile with agent_id
    let multi_agent_profile:AgentDetailSchema = askj.get_profile(&agent_id).await?;

    // update agent profile if needed
    let updated_profile:AgentProfileSchema = AgentProfileSchema {
        is_restart:Some(true),
        is_backtest_only:multi_agent_profile.is_backtest_only,
        agent_character:multi_agent_profile.agent_profile,
        agent_secret:multi_agent_profile.secret
    };
    let update_result = askj.update_profile(&agent_id,&updated_profile).await;
    match update_result {
        Ok(agent_id) => {
            println!("Updating successful, agent ID: {}", agent_id);
            println!("You can check backtest progress and result by using example of monitor_agent");
        }
        Err(err) => {
            eprintln!("Updating error: {}", err);
        }
    }

    Ok(())
}
