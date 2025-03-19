# rig-trading-kit-by-askj

> **Note**
> rig-trading-kit-by-askj depends on drift-rs which is currently only available for Linux and Mac. Windows is not yet supported.

## Introduction
rig-trading-kit-by-askj provides a rich set of trading tools for Rig-based AI agents and a framework for developers to quickly simulate and deploy their agents.

## Features
### Drift agent tools
#### Delegated (vault) trading
- Open and close perpetuals orders (long/short, market/limit) (includes mixed orders in the same transaction)
- Place TWAP orders on perpetuals (market/limit)
- Place Trailing Stop orders on perpetuals (market)
  
#### Utils
- Market indexes mapped to market names via a constant passed to agent context.
- Generate instruction hex for deposit and withdraw to vault
- Get data for vault (owner, authority, balances, etc)
- Get data on open perpetuals and spot positions and open orders.

### Rig agent as local webserver
- Easily host a webserver to prompt your Rig agents

### AskJimmy API
- Deploy customized trading agents to AskJimmy infrastructure.
- Monitor performance of deployed agents.
- Backtest strategies on AskJimmy infrastructure.
- Login and authenticate to AskJimmy multi-agent infrastructure.
- Delete deployed agent.
- Assign vaults and delegators to deployed agents.

## Minimum system requirements
- 2 GB RAM
- 15 GB disk space

## Requirements
- Rust 1.85.0
- git
- gcc
- OpenSSL
- libdrift_ffi_sys
- OpenAI API key
- Deployed Drift vault address
- Keypair for agent

### Install using install.sh
To install all requirements, run
```
sudo chmod +x install.sh && ./install.sh
```

Or, install each separately.


### Rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

### Git
```
sudo apt update
sudo apt-get install git
git --version
```

### gcc
```
sudo apt update && sudo apt install -y build-essential
```

### OpenSSL
```
sudo apt install -y pkg-config libssl-dev
```

### libdrift_ffi_sys
```
curl -L https://github.com/drift-labs/drift-ffi-sys/releases/download/v2.107.0/libdrift_ffi_sys.so -o libdrift_ffi_sys.so
sudo mv libdrift_ffi_sys.so /usr/lib/
export CARGO_DRIFT_FFI_PATH=/usr/lib
echo 'export CARGO_DRIFT_FFI_PATH=/usr/lib' >> ~/.bashrc source ~/.bashrc
```

### Environment variables
Add to ~/.bashrc to persist
```
export OPENAI_API_KEY=sk-proj-Dikmdo......
export AGENT_VAULT=2WZGt5apmJzUKTydA53dsqdnGAhEkgWt4Zuq4CjNnYt6
export AGENT_KEYPAIR=25,72,54....
```

### Deploy Drift vault
```
git clone https://github.com/drift-labs/drift-vaults.git
cd drift-vaults/ts/sdk
yarn cli init-vault --url https://solana-rpc.publicnode.com --keypair /home/user/Desktop/AskJimmy/kp --name TestVault2025.AskJimmy.Test1 --delegate 9T2BsrmnBQbZj57athD6nMjRcn5frDziQ62GehZYY5yo
```

## Quick setup
Prerequisite: deploy Drift vault with delegate and deposit some USDC to it. 
```
sudo apt update
sudo apt-get install git
git clone https://github.com/askjimmy/rig-trading-kit-by-askj.git
cd rig-trading-kit-by-askj
sudo chmod +x install.sh && ./install.sh
cargo run --release --example chat_agent
```

## Installation

Add this line to your Cargo.toml dependencies block to use the rig-trading-kit-by-askj:

```
[dependencies]
rig-trading-kit-by-askj = { git = "https://github.com/askjimmy/rig-trading-kit-by-askj.git", branch = "main" }
```

And use the tools like this:

```
use agent_trading_kit::tools::*;
use agent_trading_kit::data::MARKET_DATA;
```

```
let drift_agent = openai_client
        .agent(providers::openai::GPT_4O)
        .preamble(concat!("You are an agent designed to make autonomous trades based on user prompts. You are the delegate/executor for a Drift Vault. ",
          "Users can prompt you to make simple perpetual and spot token orders, or more complex orders involving strategies implemented in your tools",
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
        .build();
```



## Examples

### Trading chat agent

Simple agent for executing orders on behalf of a Drift vault given user prompts.

See examples/agents/chat_agent.rs for a full example.

#### Run the example

```
cargo run --release --examples chat_agent
```

#### Code

```
use std::io;
use anyhow::Result;
use rig::providers::openai;
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
    let openai_client = providers::openai::Client::from_env();

    let drift_agent = openai_client
        .agent(providers::openai::GPT_4O)
        .preamble(concat!("You are an agent designed to make autonomous trades based on user prompts. You are the delegate/executor for a Drift Vault. ",
                          "Users can prompt you to make simple perpetual and spot token orders, or more complex orders involving strategies implemented in your tools",
                          "You also have some knowledge of of existing TWAP and VWAP orders via your tools."))
        .max_tokens(1024)
        .context(MARKET_DATA.as_str())
        .tool(Deposit)
        .tool(Withdraw)
        .tool(DriftVaultInfo)
        .tool(DriftInfo)
        .tool(DriftPlaceComplexPerpOrders)
        .tool(DriftTWAPOrders)
        .tool(DriftVWAPOrders)
        .tool(DriftTrailingStopOrders)
        .tool(DriftGetOpenPositions)
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
    Ok(())
}
```

### Autonomous Agent

Users can easily create and manage their own AgenticAi agents using natural language profiles within the ASKJ ecosystem for trading on the Drift platform by using Askj SDK.
The `examples/askj` folder provides examples illustrating how to utilize the API to create autonomous trading agents.
Please check more details from [here](https://github.com/askjimmy/rig-trading-kit-by-askj/blob/main/src/askj/README.md).

#### Run the example

```
cargo run --examples list_agent
```

## Tools

### Drift
#### As vault delegate
- Place perpetual orders
- Place TWAP order
- Place Trailing Stop order

#### User/frontend utility
Both of these tools return hex encoded transaction data that the user should sign and send.
- Deposit to vault
- Withdraw from vault

#### Query information
- Get open perpetual and spot positions and open orders.
- Get market data from Drift for a given market index.
- Get vault data from Drift for a given vault public key.

## Market data
### Drift
data/markets.rs provides a string that can be passed to agents as context which maps market index to names. There is also a helper function to parse the string into a dictionary.


## Autonomous trading agent

Users can easily create and manage their own AgenticAi agents using natural language profiles within the ASKJ ecosystem for trading on the Drift platform by using Askj SDK.

The `askj` folder contains the trait for the Autonomous agent API as `agent_trading_kit::askj::Askj`. The `examples/askj` folder contains examples demonstrating how to use the API.

This API provides a convenient interface to create, deploy, and monitor autonomous trading agents on the ASKJIMMY platform.
Please check more details from [here](https://github.com/askjimmy/rig-trading-kit-by-askj/blob/main/src/askj/README.md).

## Drift CLI
A simple CLI for interacting with Drift vaults. Useful for testing.

https://github.com/drift-labs/drift-vaults/blob/master/ts/sdk/README.md

### Useful commands
#### Deploy new Drift vault 
```
yarn cli init-vault --url https://solana-rpc.publicnode.com --keypair /home/user/Desktop/AskJimmy/kp --name TestVault2025.AskJimmy.Test1 --delegate 9T2BsrmnBQbZj57athD6nMjRcn5frDziQ62GehZYY5yo
```

#### Init vault depositor
```
yarn cli init-vault-depositor --vault-address=2WZGt5apmJzUKTydA53dsqdnGAhEkgWt4Zuq4CjNnYt6 --deposit-authority=9T2BsrmnBQbZj57athD6nMjRcn5frDziQ62GehZYY5yo --url https://solana-rpc.publicnode.com --keypair /home/user/Desktop/AskJimmy/kp
```

#### Deposit to Drift vault (manager)
```
yarn cli manager-deposit --vault-address=2WZGt5apmJzUKTydA53dsqdnGAhEkgWt4Zuq4CjNnYt6 --amount=2 --url https://solana-rpc.publicnode.com --keypair /home/user/Desktop/AskJimmy/kp
```

#### Deposit to Drift vault (depositor)
Where AJauJ7PmMHCM5xsHAvwQZFbi64e4tKAjV13BD4ujQAFD represents the account created during initializeVaultDepositor
```
yarn cli deposit --vault-depositor-address=AJauJ7PmMHCM5xsHAvwQZFbi64e4tKAjV13BD4ujQAFD --amount=1 --url https://solana-rpc.publicnode.com --keypair /home/user/Desktop/AskJimmy/kp
```

# Videos

## Trailing stop order
https://www.loom.com/share/95b190db5d5340f1a987e26b665f2947?sid=8df390d2-3629-44dc-aa84-639a75daa8fe

## TWAP order
https://www.loom.com/share/cdfb1ce21c424afea14457c4a8bb8917?sid=6f5e99c5-44c9-416c-ae69-acd791c80917

## Open and close mixed perp orders
https://www.loom.com/share/d90059eaf25945e6bb85357385985bde?sid=6a4f54a1-54eb-45b6-b443-42c3e9c2af41

## Customize and deploy agent on AskJimmy
https://www.loom.com/share/7499bbdec9d3429e91042358c1c8a5e7?sid=dae4a947-3af0-4026-9020-37bf05554c52

## Deploy new vault and assign to agent
https://www.loom.com/share/8798e580c38144038b6aca073fe5a946?sid=9f55575c-6947-43bb-8437-f49c04f1a02b

## Check agent trade history
https://www.loom.com/share/11954fedd6c3443a990567b3088c5a32?sid=6f0ef374-31a5-471a-8cc4-ee174323b5dc