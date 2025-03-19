use serde_json::Value;
use anyhow::{Context, Result};
use std::collections::HashMap;

use super::request::*;
use super::schema::VaultAsignSchema;

pub async fn vault_assign_delegator(api_url: &str,oauth_key: &str,agent_id:&str) -> Result<String> {
    let assign_delegator_url = format!("{}/vault/assign_delegator/{}", api_url,agent_id);
    let query: HashMap<String, String> = HashMap::new();
    let response: Value = get_request(&assign_delegator_url, oauth_key,&query).await?;
    
    let delegator_public = response["delegator_public"].as_str().context("Invalid response from backend.")?;
    Ok(delegator_public.to_string())

}

pub async fn vault_assign_vault(api_url: &str, oauth_key: &str,agent_id:&str,vault:&VaultAsignSchema) -> Result<()> {
    let assign_vault_url = format!("{}/vault/assign_vault/{}", api_url,agent_id);
    let vault_body_json = serde_json::to_value(&vault).context("Failed to convert vault to JSON")?;
    let _response: Value = post_request(&assign_vault_url, &vault_body_json, oauth_key).await?;
    
    Ok(())

}
