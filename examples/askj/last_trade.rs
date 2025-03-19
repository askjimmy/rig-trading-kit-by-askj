use agent_trading_kit::askj::Askj;
use agent_trading_kit::askj::schema::LastTradesSchema;
use anyhow::Result;
use clap::Parser;
/**
 * Get last trades of selected agent
 * Usage: Cargo run --example last_trade --agent-id=[AGENT ID] --is-simulated=[true|false] --last-k=[number of records] --timestamp=[timestamp in miliseconds]
 */
/// Command-line arguments
#[derive(Parser)]
struct Args {
    /// Agent ID
    #[clap(long)]
    agent_id: String,
    is_simulated: Option<bool>,
    last_k: Option<i32>,
    timestamp: Option<i64>
}
#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();
    let agent_id = args.agent_id.clone();

    // Initialize the Askj struct
    let askj = Askj::new(None).await;


    //get trading performance of selected agent


    let timestamp: Option<i64> = args.timestamp.clone();
    let mut last_k: Option<i32> = args.last_k.clone();
    let is_simulated: Option<bool> = args.is_simulated.clone();
    if last_k.is_none()  {
        last_k=Some(3);
    }
    let response = askj.last_trades(&agent_id,&is_simulated,&timestamp,&last_k).await;
    match response {
        Ok(last_trades) => {
            println!("====== trades =======\n\n");
            for trade in last_trades.trades {
                println!("==== Time:{} Action:{} Price:{} Value:{} Position:{} Open Price:{} \n\n Resoning:{}\n\n",
                    trade.day,
                    trade.action,
                    trade.price,
                    trade.value,
                    trade.position,
                    trade.open_price,
                    trade.reasoning.replace("\\n", "\n")
                );
            }
            println!("====== memories =======\n\n");
            for memory in last_trades.memories {
                println!("==== Time:{} \n {} \n\n",
                    memory.created_at,
                    memory.reasoning.replace("\\n", "\n")
                );
            }
        }
        Err(err) => {
            eprintln!("Query error: {}", err);
        }
    }

    Ok(())
}
