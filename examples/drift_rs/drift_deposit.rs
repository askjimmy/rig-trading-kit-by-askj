
use std::{env, str::FromStr};
use anchor_lang::InstructionData;
use drift_rs::{constants::{self}, types::Context, DriftClient, Wallet};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction},
    message::Message, pubkey::Pubkey, signature::{Keypair, Signer}, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;

pub async fn get_deposit_message(keypair: &Keypair, drift_client: DriftClient, vault: &Pubkey, wallet: Wallet, market_index: u16, 
                          amount: u64, reduce_only: Option<bool>) 
                          -> Result<Message, anyhow::Error> {
    let program_id: Pubkey = Pubkey::from_str("vAuLTsyrvSfZRuRB3XgvkPwNGgYSs9YRYymVebLKoxR").unwrap(); // VAULT_PROGRAM_ID
    let token_mint = drift_client.get_spot_market_account(market_index).await.unwrap().mint;
    let user_token_account = get_associated_token_address(&wallet.signer(), &token_mint);
                        
    let (vault_token_account, _seed) = Pubkey::find_program_address(
              &[&b"vault_token_account"[..], vault.as_ref()],
              &program_id,
    );
                        
    let (vault_depositor_account, _seed) = Pubkey::find_program_address(
        &[&b"vault_depositor"[..], vault.as_ref(), wallet.signer().as_ref()],
        &program_id,
    );
    println!("Deposit amount: {}", amount);
    let instruction_data = InstructionData::data(&drift_rs::drift_idl::instructions::Deposit {
        market_index: market_index,
        amount: amount,
        reduce_only: reduce_only.unwrap_or(false),
    });
    
    let accounts = vec![
        AccountMeta::new(*vault, false), // Vault
        AccountMeta::new(vault_depositor_account, false), // VaultDepositor
        AccountMeta::new(wallet.signer(), true),  // Authority
        AccountMeta::new(vault_token_account, false), // Vault Token Account
        AccountMeta::new(*wallet.stats(), false), // Drift User Stats
        AccountMeta::new(wallet.default_sub_account(), false), // Drift User
        AccountMeta::new(*constants::state_account(), false), // Drift State
        AccountMeta::new(constants::derive_spot_market_vault(market_index), false), // Drift Spot Market Vault
        AccountMeta::new(user_token_account, false), // User Token Account
        AccountMeta::new_readonly(constants::PROGRAM_ID, false), // Drift Program
        AccountMeta::new_readonly(constants::TOKEN_PROGRAM_ID, false), // Token Program
        AccountMeta::new_readonly(drift_client.get_spot_market_account(market_index).await.unwrap().oracle, false), // USDC Spot Market Oracle Account
        AccountMeta::new(constants::derive_spot_market_account(market_index), false), // Drift USDC spot market
    ];
    
    let instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };
    let message = Message::new(&[instruction], Some(&keypair.pubkey()));
    Ok(message)
}

pub async fn deposit_to_vault(keypair: Keypair, drift_client: DriftClient, vault: &Pubkey, wallet: Wallet, market_index: u16, 
                          amount: u64, reduce_only: Option<bool>) 
                          -> Result<Transaction, anyhow::Error> {
    let drift_client_clone = drift_client.clone();
    let message = get_deposit_message(&keypair, drift_client_clone, vault, wallet, market_index, amount, reduce_only).await.unwrap();
    let mut transaction = Transaction::new_unsigned(message);
    let recent_blockhash = drift_client.rpc().get_latest_blockhash().await.unwrap();
    transaction.sign(&[&keypair], recent_blockhash);
    Ok(transaction)
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com"; 
    let agent_vault: String = env::var("AGENT_VAULT").unwrap();
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let secret_key_str = env::var("AGENT_KEYPAIR").expect("Missing AGENT_KEYPAIR env variable");
    let secret_key_bytes: Vec<u8> = secret_key_str
        .split(',')
        .map(|s| s.parse::<u8>().expect("Invalid byte"))
        .collect();
    let keypair = Keypair::from_bytes(&secret_key_bytes).expect("Invalid private key");

    let mut wallet: Wallet = Wallet::from(keypair.insecure_clone());
    let agent_vault_pubkey = Pubkey::from_str(&agent_vault).unwrap();
    wallet.to_delegated(agent_vault_pubkey);
    let wallet_clone = wallet.clone();
    
    let drift_client = DriftClient::new(
        Context::MainNet,
        solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url.to_string()),
        wallet_clone,
    ).await.unwrap();

    //let amount = (0.01 * 10f64.powi(6)) as u64;
    let amount: u64 = 1_000_000; // 1 USDC
    println!("Depositing {} USDC", amount);
    println!("Depositing {} USDC", (amount/10u64.pow(6)) as f64);
    let spot_market_index = 0; // USDC
    let tx = deposit_to_vault(keypair, drift_client, &agent_vault_pubkey, wallet, spot_market_index, amount, None).await.unwrap();
    match client.send_transaction(&tx) {
        Ok(signature) => println!("Transaction sent successfully! Tx Signature: {}", signature),
        Err(e) => eprintln!("Transaction failed: {:?}", e),
    }
}
