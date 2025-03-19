use agent_trading_kit::askj::Askj;
use anyhow::Result;
use chrono::{TimeZone, Utc};
mod agent_profile;
use agent_profile::initialize_profile;
use dotenv::dotenv;
use serde_json::Value;
/**
 * Backtest the agent
 * usage cargo run --example backtest
 */
 
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize the Askj struct
    // Load private key from env variable AGENT_PRIVATEKEY and log in to the ASKJIMMY backend
    // let askj = Askj::new(None).await;
    //
    // If you want to use a different private key, you can pass it in the constructor
    // let askj = Askj::new(some_private_key_string).await;
    let mut askj = Askj::new(None).await;

    match askj.login().await {
        Ok(_) => {
            // Convert the expired timestamp to a human-readable date format
            let expiration_date = Utc
                .timestamp_opt(askj.expired, 0)
                .single()
                .expect("Invalid timestamp")
                .to_rfc3339();
            println!("Login success, will expire at {}", expiration_date);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    //prepare profile of the askj agent
    let multi_agent_profile = initialize_profile()?;
    let deploy_result = askj.deploy(&multi_agent_profile).await;
    let agent_id = match deploy_result {
        Ok(agent_id) => {
            println!("Deployment successful, agent ID: {}", agent_id);
            agent_id
        }
        Err(err) => {
            eprintln!("Deployment error: {}", err);
            std::process::exit(1);
        }
    };
    println!("Awaiting backtest results...");
    //monitor backtest progress of the agent, it will stop when the backtest is completed
    let last_k: Option<i32> = None;
    let mut last_record_count = 0;
    loop {
        let response = askj.get_trading_performance(&agent_id,&last_k).await;
        let response = match response {
            Ok(agent_detail) => agent_detail,
            Err(err) => {
                eprintln!("Query error: {}", err);
                break;
            }
        };
        let agent_detail = &response.agent_detail;
        let agent_memories = &response.memories;
        let status = &agent_detail.status.status;
        
        let record_count = agent_detail.trading_simulate.len();

        if record_count > last_record_count {
            for i in (0..record_count-last_record_count).rev() {
                let trading_record = &agent_detail.trading_simulate[i];
                let reasoning = trading_record.reasoning.replace("\\n", "\n");

                let memory = &agent_memories[i];
                let market_outlook: Value = match serde_json::from_str(&memory.reasoning) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("Error: failed to parse memory.reasoning as json: {}", err);
                        continue;
                    }
                };
                let long_term_reasoning = &market_outlook["long_term_reasoning"].as_str().unwrap();
                let medium_term_reasoning = &market_outlook["medium_term_reasoning"].as_str().unwrap();
                let short_term_reasoning = &market_outlook["short_term_reasoning"].as_str().unwrap();

                println!("============== Time:{} Current Price:{} Action:{} Position:{}{}@{} Total_profit(Percent):{:.2}% Value:${}", 
                    trading_record.day,
                    trading_record.price,
                    trading_record.action,
                    trading_record.position,
                    trading_record.symbol,
                    trading_record.open_price,
                    trading_record.total_profit,
                    trading_record.value
                );

                println!("Market outlook for long-term\n{}\n\n", long_term_reasoning);
                println!("Market outlook for medium-term\n{}\n\n", medium_term_reasoning);
                println!("Market outlook for short-term\n{}\n\n", short_term_reasoning);

                println!("Reasoning of trading decision\n{}\n", reasoning);
            }
            last_record_count = agent_detail.trading_simulate.len();
            if status == "completed" || status == "stopped" {
                if status == "stopped" {
                    agent_detail.status.last_error.as_ref().map(|e| println!("Backtest stopped with error: {}", e.message));
                }else{
                    let first_trading_simulate = agent_detail.trading_simulate.last().unwrap();
                    println!("Backtested from: {}", first_trading_simulate.day);
                    let last_trading_simulate = agent_detail.trading_simulate.first().unwrap();
                    println!("Backtested to: {}", last_trading_simulate.day);

                    if let Some(performance) = &agent_detail.performance_simulate {
                        println!("Total return: {:.2}%", performance["returnsAllTime"].as_f64().unwrap()*100.0);
                        println!("Anualized return: {:.2}%", performance["annualizedReturn"].as_f64().unwrap()*100.0);
                        println!("Sharpe Ratio: {:.2}", performance["sharpeRatio"].as_f64().unwrap());
                    }

                }

                println!("\n\nBacktest completed.");
                break;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
    Ok(())

}
