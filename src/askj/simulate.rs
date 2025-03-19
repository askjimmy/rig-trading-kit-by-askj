use serde_json::Value;
use anyhow::{Context, Result};
use std::collections::HashMap;

use super::request::*;
use super::schema::AgentProfileSchema;

pub async fn simulate_new_agent(api_url: &str) -> Result<Value> {
    let new_agent_url = format!("{}/simulate/new_agent", api_url);
    let query: HashMap<String, String> = HashMap::new();
    let response: Value = get_request(&new_agent_url, "",&query).await?;
    Ok(response)

}

pub async fn simulate_deploy(api_url: &str, oauth_key: &str, profile: &AgentProfileSchema) -> Result<String> {
    let deploy_url = format!("{}/simulate/deploy", api_url);
    let deploy_body_json = serde_json::to_value(&profile).context("Failed to convert profile to JSON")?;
    let response: Value = post_request(&deploy_url, &deploy_body_json, oauth_key).await?;
    let agent_id = response["agent_id"].as_str().context("Invalid response from backend.")?;
    
    Ok(agent_id.to_string())

}

pub async fn simulate_delete(api_url: &str, oauth_key: &str,agent_id:&str) -> Result<String> {
    let delete_url = format!("{}/simulate/delete/{}", api_url,agent_id);
    let empty_json = serde_json::json!({});
    let delete_body_json = serde_json::to_value(&empty_json).context("Failed to convert profile to JSON")?;
    let _response: Value = delete_request(&delete_url, &delete_body_json, oauth_key).await?;
    
    
    Ok(agent_id.to_string())

}

pub async fn simulate_profile(api_url: &str, oauth_key: &str,agent_id:&str) -> Result<Value> {
    let profile_url = format!("{}/simulate/profile/{}", api_url,agent_id);
    let query: HashMap<String, String> = HashMap::new();

    let response = get_request(&profile_url, oauth_key,&query).await?;
    
    Ok(response)

}

pub async fn simulate_update(api_url: &str, oauth_key: &str,agent_id:&str,profile:&AgentProfileSchema) -> Result<String> {
    let update_url = format!("{}/simulate/update/{}", api_url,agent_id);
    let deploy_body_json = serde_json::to_value(&profile).context("Failed to convert profile to JSON")?;

    let response: Value = post_request(&update_url, &deploy_body_json, oauth_key).await?;
    let agent_id = response["agent_id"].as_str().context("Invalid response from backend.")?;
    
    Ok(agent_id.to_string())

}
