use agent_trading_kit::askj::Askj;
use anyhow::Result;
use clap::Parser;
/**
 * List agent from askj ecosystem
 * Usage: Cargo run --example list_agent --owner=[OWNER OF AGENT] --is-backtest-only=[true|false]
 */
/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    owner: Option<String>,
    is_backtest_only: Option<bool>,
}
#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();
    // Initialize the Askj struct
    let askj = Askj::new(None).await;

    //get trading performance of selected agent

    let owner: Option<String> = args.owner.clone();
    let is_backtest_only: Option<bool> = args.is_backtest_only.clone();

    //list all running agent 
    let response = askj.list_agent(&owner, &is_backtest_only).await;
    match response {
        Ok(list ) => {
            for agent in list.iter() {
                if let Some(performance_simulate) = &agent.performance_simulate {
                    let returns_all_time = &performance_simulate["returnsAllTime"];
                    println!("id:{} Name:{} Total Return:{:.2}%", 
                        agent.id.clone().unwrap() , 
                        agent.agent_profile.agent_name, 
                        returns_all_time.as_f64().unwrap() * 100.0);
                } else {
                    println!("id:{} Name:{} Strategy id:{}", 
                        agent.id.clone().unwrap(),
                        agent.agent_profile.agent_name, 
                        agent.agent_profile.strategy_id);
                }
            }
        }
        Err(err) => {
            eprintln!("Query error: {}", err);
        }
    }

    Ok(())
}
