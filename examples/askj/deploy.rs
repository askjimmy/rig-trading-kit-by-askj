use std::env;
use agent_trading_kit::askj::Askj;
use agent_trading_kit::askj::schema::*;
use anyhow::{Context,Result};
use chrono::{TimeZone, Utc};
mod agent_profile;
use agent_profile::initialize_profile;
use dotenv::dotenv;
use clap::Parser;
/**
 * Deploy agent with customized prompt
 * usage cargo run --example deploy json-profile
 * example : cargo run --example deploy
 * example : cargo run --example deploy --json-profile  sol_rsi_1.json
 * if set json_profile parameter, deploy by using json under jsonprofile folder,
 * otherwise deploy by using agent_profile.rs
 */

 /// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    json_profile: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

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

    //prepare profile of the askj agent
    let args = Args::parse();
    let multi_agent_profile: AgentProfileSchema = if let Some(json_profile) = args.json_profile {
        let json_profile_path = format!("examples/askj/jsonprofile/{}.json", json_profile);
        let json_text = std::fs::read_to_string(&json_profile_path).context(format!("Failed to read from file {}", json_profile_path))?;
        let mut profile:AgentProfileSchema = serde_json::from_str::<AgentProfileSchema>(&json_text).context("Failed to parse json profile")?;

        if let Some(agent_secret) = &mut profile.agent_secret {
            agent_secret.alphavantage_api_key = env::var("AGENT_PRIVATEKEY").expect("Missing AGENT_PRIVATEKEY env variable");
            agent_secret.ai.prompt.api_key = env::var("ANTHROPIC_API_KEY").expect("Missing ANTHROPIC_API_KEY env variable");
            agent_secret.ai.embedding.api_key = env::var("GEMINI_API_KEY").expect("Missing GEMINI_API_KEY env variable");
        }

        profile
    } else {
        initialize_profile()?
    };
    let deploy_result = askj.deploy(&multi_agent_profile).await;
    match deploy_result {
        Ok(agent_id) => {
            println!("Deployment successful, agent ID: {}", agent_id);
            println!("You can check backtest progress and result by using example of monitor_agent");
        }
        Err(err) => {
            eprintln!("Deployment error: {}", err);
        }
    }

    Ok(())
}
