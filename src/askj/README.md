# ASKJ SDK

Users can easily create and manage their own AgenticAi agents using natural language profiles within the ASKJ ecosystem for trading on the Drift platform by using Askj SDK.

The `src/askj` folder contains the trait for the Autonomous agent API as `agent_trading_kit::askj::Askj`. The `examples/askj` folder contains examples demonstrating how to use the API.

This API is a wrapper for using the ASKJIMMY multi-agent system.
Documentation for using the REST API can be found [here](https://api.askjimmy.xyz/api-docs).

This API provides a convenient interface to create, deploy, simulate and monitor autonomous trading agents on the ASKJIMMY platform.

Example profiles are available in the `examples/agent_profile.rs` file, serving as a foundation for developing new agents.

```
AgentProfileSchema {
    // Set to true to run backtest only, set to false to trade with real-time price
    is_backtest_only: true,
    
    // Determines the behavior when updating the profile:
    // If true, the agent will restart; if false, the updated profile will be applied without restarting.
    is_restart: false,
    
    agent_secret: Some(AgentSecretSchema {
        // API key for Alpha Vantage, used for fetching market data
        alphavantage_api_key: env::var("AGENT_PRIVATEKEY").expect("Missing AGENT_PRIVATEKEY env variable"),
        ai: AiSchema {
            prompt: PromptSchema {
                // Provider for AI prompt generation
                provider: "anthropic".to_string(),
                // Model used for AI prompt generation
                model: "claude-3-5-sonnet-20241022".to_string(),
                // API key for the AI provider
                api_key: env::var("ANTHROPIC_API_KEY").expect("Missing ANTHROPIC_API_KEY env variable"),
            },
            embedding: PromptSchema {
                // Provider for AI embedding generation
                provider: "gemini".to_string(),
                // Model used for AI embedding generation
                model: "EMBEDDING_001".to_string(),
                // API key for the AI provider
                api_key: env::var("GEMINI_API_KEY").expect("Missing GEMINI_API_KEY env variable"),
            }
        },
    }),
    agent_character: AgentCharacterSchema {
        // Name of the trading agent
        agent_name: "Demo Agent".to_string(),
        // Description of the trading agent
        description: "Trade SOL with reflection LLM multi agent".to_string(),
        
        // The ID of the strategy from the ASKJIMMY community that provides the signal used as 
        // a prompt context by the autonomous trading agent.
        // You can find the strategy in https://solana.askjimmy.xyz or select one from https://www.askjimmy.xyz/strategies.
        // You must select no portfolio strategy, no rebalance strategy.
        // The strategy ID can be found in the URL of the strategy page in the ASKJIMMY community.
        // For example, the strategy ID for the strategy at https://www.askjimmy.xyz/strategy/24f234ca-d9a6-4770-9f6d-2e518d374eac is 24f234ca-d9a6-4770-9f6d-2e518d374eac.
        strategy_id: "24f234ca-d9a6-4770-9f6d-2e518d374eac".to_string(),

        // If None, predefined placeholders from the ASKJIMMY system will be used.
        // These placeholders help generate prompts.
        placeholders: PlaceholdersSchema {
            // Placeholder for usage of ASKJIMMY strategy
            usage_askjimmy_strategy: Some("
                As a moderate trader, you aim for a balance between risk and reward. 
                You may consider longer holding periods, 
                not rushing to buy and sell, and placing greater emphasis 
                on researching the crypto's fundamentals and technical analysis.
                ".to_string()),
            // Placeholder for trader reference
            trader_preference: Some("
                Only execute trades that align with both the strategy's signal and 
                your own analysis.
                Execute a buy order only if the strategy signals a long position 
                and your analysis indicates a bullish market.
                Execute a sell order only if the strategy signals 
                a short position and your analysis indicates a bearish market.
                Do not open a position if the strategy's 
                signal contradicts your analysis. In such cases, 
                ignore the signal.".to_string()),
            // Placeholder for decision prompt analysis
            decision_prompt_analysis: None,
            // Placeholder for decision prompt reasoning
            decision_prompt_reasoning: None,
            // Placeholder for decision prompt
            decision_prompt: None,
            // Placeholder for decision task description trading
            decision_task_description_trading: None,
            // Placeholder for high-level reflection prompt trading
            high_level_reflection_prompt_trading: None,
            // Placeholder for high-level reflection task description trading
            high_level_reflection_task_description_trading: None,
            // Placeholder for low-level reflection effects trading
            low_level_reflection_effects_trading: None,
            // Placeholder for low-level reflection prompt trading
            low_level_reflection_prompt_trading: None,
            // Placeholder for low-level reflection task description trading
            low_level_reflection_task_description_trading: None,
            // Placeholder for market intelligence effects trading
            market_intelligence_effects_trading: None,
            // Placeholder for market intelligence latest summary prompt trading
            market_intelligence_latest_summary_prompt_trading: None,
            // Placeholder for market intelligence past summary prompt trading
            market_intelligence_past_summary_prompt_trading: None,
            // Placeholder for market intelligence task description trading usage ASKJIMMY strategy
            market_intelligence_task_description_tradingusage_askjimmy_strategy: None,
            // Placeholder for professional guidance
            professional_guidance: None,
        },

        // Number of bars to backtest, currently limited to 10 bars by the ASKJIMMY ecosystem.
        back_step: 10,
        // The number of days to look back for past price to predict future price in short term
        short_term_past_date_range: 6,
        // The number of days to look back for past price to predict future price in medium term
        medium_term_past_date_range: 28,
        // The number of days to look back for past price to predict future price in long term
        long_term_past_date_range: 56,

        // The number of days to look back for previous actions
        previous_action_look_back_days: 56,
        // The number of top k to select from reflection, market intelligence, and low-level reflection
        top_k: 5,

    }
}
```

## `agent_trading_kit::askj::Askj`

The `Askj` trait provides various methods to interact with the ASKJIMMY backend. Below is a list of its members and their descriptions:

### Methods

- `new(private_key: Option<String>) -> Self`
  - Creates a new instance of the `Askj` struct.

- `login(&mut self) -> Result<()>`
  - Logs in to the ASKJIMMY backend using the private key and OAuth key.

- `get_predefined_placeholders(&self) -> Result<Value>`
  - Retrieves predefined placeholders.

- `deploy(&self, profile: &AgentProfileSchema) -> Result<String>`
  - Deploys a new agent with the given profile.

- `get_profile(&self, agent_id: &str) -> Result<AgentDetailSchema>`
  - Retrieves the profile of the agent with the given ID.

- `update_profile(&self, agent_id: &str, profile: &AgentProfileSchema) -> Result<String>`
  - Updates the profile of the agent with the given ID.

- `delete_agent(&self, agent_id: &str) -> Result<String>`
  - Deletes the agent with the given ID.

- `list_agent(&self, owner: &Option<String>, is_backtest_only: &Option<bool>) -> Result<Vec<AgentDetailWithMetrics>>`
  - Lists all agents for the given owner and backtest status.

- `get_trading_performance(&self, agent_id: &str, last_k: &Option<i32>) -> Result<AgentDetailWithMetricsTradingMemories>`
  - Retrieves the trading performance of the agent with the given ID.

- `last_trades(&self, agent_id: &str, is_simulated: &Option<bool>, timestamp: &Option<i64>, last_k: &Option<i32>) -> Result<LastTradesSchema>`
  - Retrieves the last trades of the agent with the given ID.

- `assign_delegator(&self,agent_id:&str) -> Result<String>`
  - Assign a delegator to selected agent.

- `assign_vault(&self,agent_id:&str,vault:&VaultAsignSchema) -> Result<()>`
  - Assign a trading vault from the Drift on the selected agent.

## Examples

The `examples/askj` folder contains various examples demonstrating how to use the API. Below is a list of the examples:

- `agent_profile.rs`
  - Demonstrates how to initialize and manage an agent profile.

- `backtest.rs`
  - Demonstrates how to run a backtest for an agent.

- `delete_agent.rs`
  - Demonstrates how to delete an agent.

- `deploy.rs`
  - Demonstrates how to deploy a new agent.

- `last_trade.rs`
  - Demonstrates how to retrieve the last trades of an agent.

- `list_agent.rs`
  - Demonstrates how to list all agents.

- `monitor_agent.rs`
  - Demonstrates how to monitor the performance of an agent.

- `predefined_placeholder.rs`
  - Demonstrates how to retrieve predefined placeholders.

- `update_agent.rs`
  - Demonstrates how to update the profile of an agent.

- `assgin_delegator.rs`
  - Demonstrates how to assign a delegator of an agent.

- `assign_vault.rs`
  - Demonstrates how to assign trading vault from the Drift to selected agent.

## Getting Started

The following environment variables must be set in an `.env` file located in the `examples/askj` folder:

- `ALPHAVANTAGE_API_KEY`: The API key for Alphavantage.
- `ANTHROPIC_API_KEY`: The API key for Anthropics.
- `GEMINI_API_KEY`: The API key for Gemini.
- `AGENT_PRIVATEKEY`: The private key of the Solana wallet to use for authentication.

All fields are mandatory.

To get started, you can run any of the examples using the following command:

```sh
cargo run --example backtest
```
