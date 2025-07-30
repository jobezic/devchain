use std::{fs::File, io::{BufReader, Write}};

use crate::{blockchain::{Blockchain, Transaction}, wallet::Wallet};
use clap::{Parser, Subcommand};

mod blockchain;
mod wallet;

#[derive(Parser)]
#[command(name = "rustcoin")]
#[command(about = "A simple blockchain wallet", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show your wallet address
    Address,
    /// Show your current balance
    Balance,
    /// Send coins to someone
    Send {
        to: String,
        amount: u64,
    },
}

fn main() {
    
    let cli = Cli::parse();
    let wallet = Wallet::load_or_create("wallet.dat");
    let mut blockchain = load_or_create_blockchain(&wallet.address()).unwrap();

    match &cli.command {
        Commands::Address => {
            println!("📬 Your address: {}", wallet.address());
        }
        Commands::Balance => {
            let balances = blockchain.get_balances();
            println!("{:#?}", balances);
            let balance = balances.get(&wallet.address()).unwrap_or(&0);
            println!("💰 Your balance: {}", balance);
        }
        Commands::Send { to, amount } => {
            let tx = Transaction {
                from: wallet.address(),
                to: to.clone(),
                amount: *amount,
                signature: None,
                public_key: None,
            };

            let signed_tx = tx.sign(&wallet);

            if let Ok(_) = blockchain.add_block(Some(signed_tx.clone())) {
                println!("✅ Transaction added to pool: {:?}", signed_tx);
            } else {
                println!("❌ Not enough funds or invalid transaction");
            }
        }
    }

    save_blockchain(blockchain).unwrap();
}

fn load_or_create_blockchain(miner_address: &str) -> anyhow::Result<Blockchain> {
    let blockchain;
    if let Ok(file) = File::open("blockchain.json") {
        let reader = BufReader::new(file);
        blockchain = serde_json::from_reader(reader)?;
    } else {
        blockchain = Blockchain::new(miner_address);
    }

    Ok(blockchain)
}

fn save_blockchain(blockchain: Blockchain) -> anyhow::Result<()> {
    let json = serde_json::to_string(&blockchain)?;
    let mut file = File::create("blockchain.json")?;
    file.write_all(json.as_bytes()).or_else(|_| Err(anyhow::anyhow!("Could not possible to save the blockchain!")))
}
