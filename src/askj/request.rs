use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde_json::{json,Value};
use std::collections::HashMap;

pub async fn get_request(url: &str, oauth_key: &str,query: &HashMap<String, String>) -> Result<Value> {
    let client = Client::new();
    let response = client
        .get(url)
        .query(query)
        .header("Authorization", format!("Bearer {}", oauth_key))
        .send()
        .await
        .context("Failed to send GET request")?;

        if response.status().is_success() {
            let value = response
                .json::<Value>()
                .await
                .context("Failed to parse POST response")?;
    
            Ok(value)
        } else {
            let error_response = match response.text().await {
                Ok(text) => serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "error": text })),
                Err(_) => json!({ "error": "Failed to parse error response".to_string() }),
            };
        
            let err = error_response
                .get("error")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "No error message in response".to_string());
    
            Err(anyhow!(err))
        }
    
}

pub async fn post_request(url: &str, body: &Value, oauth_key: &str) -> Result<Value> {
    let client = Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", oauth_key))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(body)?)
        .send()
        .await
        .context("Failed to send POST request")?;

    if response.status().is_success() {
        let value = response
            .json::<Value>()
            .await
            .context("Failed to parse POST response")?;

        Ok(value)
    } else {
        let error_response = match response.text().await {
            Ok(text) => serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "error": text })),
            Err(_) => json!({ "error": "Failed to parse error response".to_string() }),
        };

        let err = error_response
            .get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No error message in response".to_string());

        Err(anyhow!(err))
    }
}

pub async fn delete_request(url: &str, body: &Value, oauth_key: &str) -> Result<Value> {
    let client = Client::new();
    let response = client
        .delete(url)
        .header("Authorization", format!("Bearer {}", oauth_key))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(body)?)
        .send()
        .await
        .context("Failed to send DELETE request")?;

    if response.status().is_success() {
        let value = response
            .json::<Value>()
            .await
            .context("Failed to parse DELETE response")?;

        Ok(value)
    } else {
        let error_response = match response.text().await {
            Ok(text) => serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "error": text })),
            Err(_) => json!({ "error": "Failed to parse error response".to_string() }),
        };

        let err = error_response
            .get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No error message in response".to_string());

        Err(anyhow!(err))
    }
}