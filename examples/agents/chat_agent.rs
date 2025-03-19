use std::io;
use anyhow::Result;
use rig::{
    completion::Prompt,
    providers,
};
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};
use std::io::Write;

use agent_trading_kit::tools::*;
use agent_trading_kit::data::MARKET_DATA;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    const MAX_DEBUG_LEVEL:tracing::Level = tracing::Level::ERROR;
    tracing_subscriber::fmt()
        .with_max_level(MAX_DEBUG_LEVEL)
        .with_target(false)
        .init();

    let openai_client = providers::openai::Client::from_env();

    let drift_agent = openai_client
        .agent(providers::openai::GPT_4O)
        .preamble(concat!("You are an agent designed to make autonomous trades based on user prompts. You are the delegate/executor for a Drift Vault. ",
                          "Users can prompt you to open and close simple/mixed perpetual and spot token orders, or more complex orders involving strategies implemented in your tools",
                          "You also have some knowledge of of existing TWAP and VWAP orders via your tools."))
        .max_tokens(1024)
        .context(MARKET_DATA.as_str())
        .tool(Deposit)
        .tool(Withdraw)
        .tool(DriftVaultInfo)
        .tool(DriftInfo)
        .tool(DriftPlacePerpOrders)
        .tool(DriftTWAPOrders)
        .tool(DriftVWAPOrders)
        .tool(DriftTrailingStopOrders)
        .tool(DriftGetOpenPositions)
        .tool(DriftClosePerpPosition)
        .build();

    println!("[Example Agent]");
    println!("Enter your prompt below");
    loop {
        println!("[Enter prompt] > ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input) 
            .expect("Failed to read line"); 
        let input = input.trim();
        let response = drift_agent.prompt(input).await?;
        let formatted_response = response.replace("\\n", "\n").replace("\\\"", "\"");
        println!("Agent: {}", formatted_response);

    }
}
