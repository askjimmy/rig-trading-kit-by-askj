use serde::{Deserialize, Serialize};
use serde_json::Value;



#[derive(Deserialize, Serialize, Debug,Clone)]

pub struct AgentDetailSchema {
    
    /// Set to true to run backtest only, set to false to trade with real-time price
    pub is_backtest_only: bool,
    pub agent_profile:AgentCharacterSchema,
    pub status:AgentStatusSchema,
    

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault:Option<VaultSchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<AgentSecretSchema>,

    pub created_at:String,
    pub updated_at:String,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentProfileSchema {
    /// Determines the behavior when updating the profile:
    /// If true, the agent will restart; if false, the updated profile will be applied without restarting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restart: Option<bool>,
    
    /// Set to true to run backtest only, set to false to trade with real-time price
    pub is_backtest_only: bool,
    
    /// Contains the character details of the agent
    pub agent_character: AgentCharacterSchema,
    
    /// Contains the secret details of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_secret: Option<AgentSecretSchema>,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentCharacterSchema {
    /// Name of the trading agent
    pub agent_name: String,
    
    /// Description of the trading agent
    pub description: String,
    
    /// The ID of the strategy from the ASKJIMMY community that provides the signal used as a prompt context by the autonomous trading agent.
    pub strategy_id: String,
    
    /// The timeframe used for trading. This feature will be supported in the future, but is currently unsupported.
    pub timeframe: String,
    
    /// The ticker used for trading. This feature will be supported in the future, but is currently unsupported.
    pub symbol: String,
    
    /// Contains various placeholders used for generating prompts
    pub placeholders: PlaceholdersSchema,
    
    /// Number of bars to backtest, currently limited to 10 bars by the ASKJIMMY ecosystem.
    pub back_step: u32,
    
    /// Number of bars to train, currently not supported by the ASKJIMMY ecosystem.
    /// #[serde(skip_serializing_if = "Option::is_none")]
    pub training_step: Option<u32>,
    
    /// The number of days to look back for past price to predict future price in short term
    pub short_term_past_date_range: u32,
    
    /// The number of days to look back for past price to predict future price in medium term
    pub medium_term_past_date_range: u32,
    
    /// The number of days to look back for past price to predict future price in long term
    pub long_term_past_date_range: u32,
    
    /// The number of days to look at future price in short term while training
    pub short_term_next_date_range: u32,
    
    /// The number of days to look at future price in medium term while training
    pub medium_term_next_date_range: u32,
    
    /// The number of days to look at future price in long term while training
    pub long_term_next_date_range: u32,
    
    /// The number of days to look back for previous actions
    pub previous_action_look_back_days: u32,
    
    /// The number of top k to select from reflection, market intelligence, and low-level reflection
    pub top_k: u32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct PlaceholdersSchema {
    /// Placeholder for usage of ASKJIMMY strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_askjimmy_strategy: Option<String>,
    
    /// Placeholder for trader reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trader_preference: Option<String>,
    
    /// Placeholder for decision prompt analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_prompt_analysis: Option<String>,
    
    /// Placeholder for decision prompt reasoning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_prompt_reasoning: Option<String>,
    
    /// Placeholder for decision prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_prompt: Option<String>,
    
    /// Placeholder for decision task description trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_task_description_trading: Option<String>,
    
    /// Placeholder for high-level reflection prompt trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_level_reflection_prompt_trading: Option<String>,
    
    /// Placeholder for high-level reflection task description trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_level_reflection_task_description_trading: Option<String>,
    
    /// Placeholder for low-level reflection effects trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_level_reflection_effects_trading: Option<String>,
    
    /// Placeholder for low-level reflection prompt trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_level_reflection_prompt_trading: Option<String>,
    
    /// Placeholder for low-level reflection task description trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_level_reflection_task_description_trading: Option<String>,
    
    /// Placeholder for market intelligence effects trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_intelligence_effects_trading: Option<String>,
    
    /// Placeholder for market intelligence latest summary prompt trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_intelligence_latest_summary_prompt_trading: Option<String>,
    
    /// Placeholder for market intelligence past summary prompt trading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_intelligence_past_summary_prompt_trading: Option<String>,
    
    /// Placeholder for market intelligence task description trading usage ASKJIMMY strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_intelligence_task_description_tradingusage_askjimmy_strategy: Option<String>,
    
    /// Placeholder for professional guidance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub professional_guidance: Option<String>,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentSecretSchema {
    /// API key for Alpha Vantage, used for fetching market data
    pub alphavantage_api_key: String,
    
    /// Contains AI-related configurations
    pub ai: AiSchema,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AiSchema {
    /// Contains prompt-related configurations
    pub prompt: PromptSchema,
    
    /// Contains embedding-related configurations
    pub embedding: PromptSchema,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct PromptSchema {
    /// Provider for AI prompt generation
    pub provider: String,
    
    /// Model used for AI prompt generation
    pub model: String,
    
    /// API key for the AI provider
    pub api_key: String,
}


#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct VaultSchema {

    /// Vault address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<String>,
    
    /// Vault name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    
    /// Delegator of vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_delegator: Option<String>,

    /// vault status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// txhash used to create vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txhash: Option<String>
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentStatusSchema {

    /// agent status , one of created, updated, running, completed, stopped 
    pub status: String,
    
    /// last error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<ErrorSchema>,
    
    /// last time to get error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lasttime: Option<String>,

    /// running backtest step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_in: Option<i32>
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct ErrorSchema {

    /// message for error
    pub message: String,
    
    /// time of error
    pub timestamp: String,
    
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentTradingRecordItem {
    /// The symbol of the traded asset
    pub symbol: String,
    /// The time of the trade with ISO time string
    pub day: String,
    /// The equity of the account with USD value
    pub value: f64,
    /// The cash for the account
    pub cash: f64,
    /// The position size of the trade
    pub position: f64,
    /// The today return of the trade with percent rate, 0.1 is 10%
    pub ret: f64,
    /// The price of the asset at the time of trade
    pub price: f64,
    /// The total profit from the trade by percent rate value, 0.1 is 10%
    pub total_profit: f64,
    /// The total return from the trade by percent rate value, 0.1 is 10%
    pub total_return: f64,
    /// The floating profit of the trade with percent rate value, 0.1 is 10%
    pub floating_profit: f64,
    /// The opening price of the asset
    pub open_price: f64,
    /// Action in this time, BUY/SELL/EXIT/HOLD
    pub action: String,
    /// Reasoning for action
    pub reasoning: String,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct AgentDetailWithMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Agent profile
    pub agent_profile: AgentCharacterSchema,
    /// Vault details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault: Option<VaultSchema>,
    /// Agent status
    pub status: AgentStatusSchema,
    /// Owner of the agent as wallet address
    pub owner: String,
    /// Time created the agent
    pub created_at: String,
    /// Time updated the agent
    pub updated_at: String,
    /// Simulate performance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_simulate: Option<Value>,
    /// Live performance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_vault: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgentDetailWithMetricsTrading {
    /// Agent profile
    pub agent_profile: AgentCharacterSchema,
    /// Vault details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault: Option<VaultSchema>,
    /// Agent status
    pub status: AgentStatusSchema,
    /// Owner of the agent as wallet address
    pub owner: String,
    /// Time created the agent
    pub created_at: String,
    /// Time updated the agent
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_simulate: Option<Value>,
    /// Live performance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_vault: Option<Value>,
    /// Simulated trading records
    pub trading_simulate: Vec<AgentTradingRecordItem>,
    /// Live trading records
    pub trading_vault: Vec<AgentTradingRecordItem>,

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgentDetailWithMetricsTradingMemories {
    pub agent_detail:AgentDetailWithMetricsTrading,
    pub memories: Vec<MemorySchema>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MemorySchema {
    pub reasoning: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LastTradesSchema {
    pub trades:Vec<AgentTradingRecordItem>,
    pub memories: Vec<MemorySchema>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VaultAsignSchema {
    pub txhash:String,
    pub vault_name: String,
    pub vault_address: String
}
