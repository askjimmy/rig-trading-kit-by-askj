use serde_json::Value;
use anyhow::Result;
use super::request::*;
use std::collections::HashMap;

pub async fn monitor_detail(api_url: &str, agent_id:&str,last_k:&Option<i32>) -> Result<Value> {
    let detail_url = format!("{}/monitor/detail/{}", api_url,agent_id);
    let mut query: HashMap<String, String> = HashMap::new();

    if let Some(k) = last_k {
        query.insert("last_k".to_string(),k.to_string());
    }
    let detail_response: Value = get_request(&detail_url, "",&query).await?;
    Ok(detail_response)

}


pub async fn monitor_last_trades(api_url: &str, agent_id:&str,is_simulated:&Option<bool>,last_k:&Option<i32>,timestamp:&Option<i64>) -> Result<Value> {

    let last_url = format!("{}/monitor/last_trades/{}", api_url,agent_id);
    let mut query: HashMap<String, String> = HashMap::new();
    if let Some(k) = last_k {
        query.insert("k".to_string(),k.to_string());
    }
    if let Some(t) = timestamp {
        query.insert("time".to_string(),t.to_string());
    }
    if let Some(v) = is_simulated {
        query.insert("time".to_string(),v.to_string());
    }
    
    let detail_response: Value = get_request(&last_url, "",&query).await?;
    
    Ok(detail_response)

}


pub async fn monitor_list(api_url: &str, owner:&Option<String>,is_backtest_only:&Option<bool>) -> Result<Value> {

    let list_url = format!("{}/monitor/list", api_url);

    let mut query: HashMap<String, String> = HashMap::new();
    if let Some(v) = owner {
        query.insert("owner".to_string(),v.to_string());
    }
    if let Some(v) = is_backtest_only {
        query.insert("is_backtest_only".to_string(),v.to_string());
    }

    let response: Value = get_request(&list_url,"",&query).await?;
    
    Ok(response)

}
