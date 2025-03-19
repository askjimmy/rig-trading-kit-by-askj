mod request;
mod auth;
mod simulate;
mod monitor;
mod vault;

pub mod schema;

use anyhow::Result;
use schema::*;
use serde_json::Value;
use vault::vault_assign_delegator;
use vault::vault_assign_vault;
use crate::askj::auth::auth_login;
use crate::askj::simulate::*;
use crate::askj::monitor::*;

pub struct Askj {
    pub oauth_key: String,
    pub expired: i64,
    pub api_url: String,
    pub private_key:String
}

impl Askj {
    pub async fn new(private_key:Option<String>) -> Self {

        Askj { 
            api_url: "https://api.askjimmy.xyz".to_string(),
            oauth_key: "".to_string(),
            private_key:private_key.unwrap_or_default(),
            expired:0
        }
    }

    /// Logs in to the ASKJIMMY backend using the private key loaded from the environment variable AGENT_PRIVATEKEY
    /// and the oauth key provided in the constructor of this struct.
    pub async fn login(&mut self) -> Result<()> {

        let (token,expire) = auth_login(&self.private_key,&self.api_url,&self.oauth_key).await?;
        self.oauth_key = token;
        self.expired = chrono::Utc::now().timestamp()+expire;

        Ok(())
    }

    ///============================= simulator
    //get predefined place holders
    pub async fn get_predefined_placeholders(&self) -> Result<Value> {

        let predefined_placeholders= simulate_new_agent(&self.api_url).await?;

        Ok(predefined_placeholders)
    }

    //deploy agent
    pub async fn deploy(&self,profile:&AgentProfileSchema) -> Result<String> {

        let agent_id = simulate_deploy(&self.api_url,&self.oauth_key,profile).await?;

        Ok(agent_id)
    }

    //get agent profile by agent_id
    pub async fn get_profile(&self,agent_id:&str) -> Result<AgentDetailSchema> {

        let response = simulate_profile(&self.api_url,&self.oauth_key,agent_id).await?;

        let agent_profile:AgentDetailSchema = serde_json::from_value(response)?;
        
        Ok(agent_profile)
    }

    //update agent profile of agent_id
    pub async fn update_profile(&self,agent_id:&str,profile:&AgentProfileSchema) -> Result<String> {

        let agent_id = simulate_update(&self.api_url,&self.oauth_key,agent_id,profile).await?;
        
        Ok(agent_id)
    }

    //delete agent profile with agent_id
    pub async fn delete_agent(&self,agent_id:&str) -> Result<String> {

        let agent_id=simulate_delete(&self.api_url,&self.oauth_key,agent_id).await?;
        Ok(agent_id)
        
    }

    ///======================== monitor
    //list agent
    pub async fn list_agent(&self,owner:&Option<String>,is_backtest_only:&Option<bool>) -> Result<Vec<AgentDetailWithMetrics>> {

        let response = monitor_list(&self.api_url,owner,is_backtest_only).await?;
        let list: Vec<AgentDetailWithMetrics> = serde_json::from_value(response)?;

        Ok(list)
    }
    //get trading performance of selected agent
    pub async fn get_trading_performance(&self,agent_id:&str,last_k:&Option<i32>) -> Result<AgentDetailWithMetricsTradingMemories> {

        let detail_response = monitor_detail(&self.api_url,agent_id,last_k).await?;
        let detail: AgentDetailWithMetricsTradingMemories = serde_json::from_value(detail_response)?;

        Ok(detail)
    }

    //get last trades of selected agent
    pub async fn last_trades(&self,agent_id:&str,is_simulated:&Option<bool>,timestampe:&Option<i64>,last_k:&Option<i32>) -> Result<LastTradesSchema> {

        let last_trades = monitor_last_trades(&self.api_url,agent_id,is_simulated,last_k,timestampe).await?;
        let last_trades: LastTradesSchema = serde_json::from_value(last_trades)?;
        Ok(last_trades)
    }

    ///======================== monitor
    //assign delegator to selected agent
    pub async fn assign_delegator(&self,agent_id:&str) -> Result<String> {

        let delegator = vault_assign_delegator(&self.api_url,&self.oauth_key,agent_id).await?;

        Ok(delegator)
    }

    //assign delegator to selected agent
    pub async fn assign_vault(&self,agent_id:&str,vault:&VaultAsignSchema) -> Result<()> {

        vault_assign_vault(&self.api_url,&self.oauth_key,agent_id,vault).await?;

        Ok(())
    }
}

