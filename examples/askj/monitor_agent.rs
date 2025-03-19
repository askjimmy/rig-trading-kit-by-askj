use agent_trading_kit::askj::Askj;
use anyhow::Result;
use clap::Parser;

/**
 * Monitor selected agent with agent_id
 * Usage: Cargo run --example monitor_agent --agent-id=[YOUR AGENT ID] --last-k=[number of records]
 */

/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    agent_id: String,
    last_k:Option<i32>
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();
    let agent_id = args.agent_id.clone();

    // Initialize the Askj struct
    let askj = Askj::new(None).await;

    //get trading performance of selected agent
    let last_k: Option<i32> = args.last_k.clone();
    let response = askj.get_trading_performance(&agent_id,&last_k).await;
    match response {
        Ok(agent_detail_memory) => {
            let agent_detail = &agent_detail_memory.agent_detail;
            if agent_detail.performance_simulate.is_some() {
                let performance = agent_detail.performance_simulate.clone().unwrap();
                println!("Name:{} Total Return:{:.2}%", agent_detail.agent_profile.agent_name, performance["returnsAllTime"].as_f64().unwrap() * 100.0);
            }else{
                println!("Name:{} Strategy id:{}", agent_detail.agent_profile.agent_name, agent_detail.agent_profile.strategy_id);
            }
        }
        Err(err) => {
            eprintln!("Query error: {}", err);
        }
    }

    Ok(())
}
