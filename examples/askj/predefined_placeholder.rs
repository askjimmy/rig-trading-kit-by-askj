use agent_trading_kit::askj::Askj;
use anyhow::Result;
/**
 * Deploy agent with customized prompt
 */
#[tokio::main]
async fn main() -> Result<()> {

    // Initialize the Askj struct
    let askj = Askj::new(None).await;

    // get predefined placeholder
    let result = askj.get_predefined_placeholders().await;
    match result {
        Ok(predefined_placeholders) => {    
            println!("===== Listing predefined placeholders for customizing prompts.");
            for (key, value) in predefined_placeholders["system_placeholders"].as_object().unwrap() {
                println!("{}\n", key);
                println!("{}\n\n", value.as_str().unwrap().replace('\n', "\n"));
            }
            println!("===== List predefined trader references for customizing profiles.");
            for (key, value) in predefined_placeholders["system_trader_reference"].as_object().unwrap() {
                println!("{}\n", key);
                println!("{}\n\n", value.as_str().unwrap().replace('\n', "\n"));
            }
            
        }
        Err(err) => {
            eprintln!("Server error: {}", err);
        }
    }

    Ok(())
}