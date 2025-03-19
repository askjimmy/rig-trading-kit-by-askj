use std::env;
use agent_trading_kit::askj::schema::*;
use anyhow::Result;

pub fn initialize_profile() -> Result<AgentProfileSchema> {
    Ok(AgentProfileSchema {
        // Set to true to run backtest only, set to false to trade with real-time price
        is_backtest_only: true,
        
        // Determines the behavior when updating the profile:
        // If true, the agent will restart; if false, the updated profile will be applied without restarting.
        is_restart: Some(false),
        
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
            agent_name: "SOL trader".to_string(),
            // Description of the trading agent
            description: "Trade SOL with reflection LLM multi agent".to_string(),
            
            // The ID of the strategy from the ASKJIMMY community that provides the signal used as a prompt context by the autonomous trading agent.
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
                    As a moderate trader, you aim for a balance between risk and reward. You may consider longer holding periods, 
                    not rushing to buy and sell, and placing greater emphasis on researching the crypto's fundamentals and technical analysis.
                    ".to_string()),
                // Placeholder for trader reference
                trader_preference: Some("
                    Only execute trades that align with both the strategy's signal and your own analysis.
                    Execute a buy order only if the strategy signals a long position and your analysis indicates a bullish market.
                    Execute a sell order only if the strategy signals a short position and your analysis indicates a bearish market.
                    Do not open a position if the strategy's signal contradicts your analysis. In such cases, ignore the signal.".to_string()),
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
    
            // Number of bars to backtest, currently limited to 20 bars by the ASKJIMMY ecosystem.
            back_step: 5,
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
    
            // The timeframe and ticker used for trading. This feature will be supported in the future, but is currently unsupported.
            timeframe: "4h".to_string(),
            symbol: "SOL".to_string(),

            // The following features are currently unsupported by the ASKJIMMY ecosystem.
    
            // Number of bars to train, currently not supported by the ASKJIMMY ecosystem.
            training_step: Some(20),
            // The number of days to look at future price in short term while training
            short_term_next_date_range: 6,
            // The number of days to look at future price in medium term while training
            medium_term_next_date_range: 28,
            // The number of days to look at future price in long term while training
            long_term_next_date_range: 56,
        }
    })
}
