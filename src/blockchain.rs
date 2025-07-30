use anyhow::anyhow;
use sha2::{Digest, Sha256};
use chrono::Utc;
use std::{collections::HashMap, time::Instant};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signature, PublicKey, Verifier};

use crate::wallet::Wallet;

const BLOCK_REWARD: u64 = 50; // Reward per block in "coins"
const DIFFICULTY: usize = 4; // Mining difficulty

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Option<String>,
    pub public_key: Option<String>,
}

impl Transaction {
    pub fn sign(mut self, wallet: &Wallet) -> Self {
        let tx_data = format!("{}{}{}", self.from, self.to, self.amount);
        let message = tx_data.as_bytes();

        let signature = wallet.sign(message);
        let signature_hex = hex::encode(signature.to_bytes());
        let public_key_hex = hex::encode(wallet.keypair.public.as_bytes());

        self.signature = Some(signature_hex);
        self.public_key = Some(public_key_hex);

        self
    }

    pub fn verify_transaction_signature(&self) -> bool {
        let Some(sig_hex) = &self.signature else { return false; };
        let Some(pk_hex) = &self.public_key else { return false; };
    
        let tx_data = format!("{}{}{}", self.from, self.to, self.amount);
        let message = tx_data.as_bytes();
    
        let sig_bytes = match hex::decode(sig_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };
    
        let pk_bytes = match hex::decode(pk_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };
    
        let signature = match Signature::from_bytes(&sig_bytes) {
            Ok(s) => s,
            Err(_) => return false,
        };
    
        let public_key = match PublicKey::from_bytes(&pk_bytes) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
    
        public_key.verify(message, &signature).is_ok()
    }
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
    transactions: Vec<Transaction>,
}

impl Block {
    fn new(index: u64, miner_address: &str, previous_hash: String, difficulty: usize, transaction: Option<Transaction>) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash;

        // Create the reward transaction (coinbase)
        let mut transactions = vec![];
        if let Some(extra_data) = transaction {
            transactions.push(extra_data);
        } else {
            transactions.push(Transaction {
                from: "COINBASE".to_string(),
                to: miner_address.to_string(),
                amount: BLOCK_REWARD,
                signature: None,
                public_key: None,
            });
        }

        let start = Instant::now();

        loop {
            let content = format!(
                "{}{}{:?}{}{}",
                index,
                timestamp,
                transactions,
                previous_hash,
                nonce
            );
            hash = calculate_hash(&content);

            if hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }

            nonce += 1;
        }

        let duration = start.elapsed();
        println!(
            "⛏️  Mined block {} in {:.2?} (nonce: {}, hash: {})",
            index, duration, nonce, hash
        );

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    reward_address: String,
}

impl Blockchain {
    pub fn new(miner_address: &str) -> Self {
        let genesis = Block::new(0, miner_address, "0".into(), DIFFICULTY, None);
        Blockchain {
            chain: vec![genesis],
            difficulty: DIFFICULTY,
            reward_address: miner_address.to_string(),
        }
    }

    pub fn add_block(&mut self, transaction: Option<Transaction>) -> anyhow::Result<()> {
        if let Some(transaction) = transaction.clone() {
            let balances = self.get_balances();
            if !is_transaction_valid(&transaction, &balances) {
                return Err(anyhow!("transaction: {:#?} is not valid", transaction));
            }
        }

        let last = self.chain.last().unwrap();
        let new_block = Block::new(
            last.index + 1,
            &self.reward_address,
            last.hash.clone(),
            self.difficulty,
            transaction,
        );
        self.chain.push(new_block);

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let content = format!(
                "{}{}{:?}{}{}",
                current.index,
                current.timestamp,
                current.transactions,
                current.previous_hash,
                current.nonce
            );

            if current.hash != calculate_hash(&content)
                || !current.hash.starts_with(&"0".repeat(self.difficulty))
            {
                return false;
            }
        }
        true
    }    

    pub fn get_balances(&self) -> HashMap<String, i64> {
        let mut balances = HashMap::new();

        for block in &self.chain {
            for tx in &block.transactions {
                let from_balance = balances.entry(tx.from.clone()).or_insert(0);
                *from_balance -= tx.amount as i64;

                let to_balance = balances.entry(tx.to.clone()).or_insert(0);
                *to_balance += tx.amount as i64;
            }
        }

        balances
    }

    pub fn to_blocks(&self) -> Vec<Block> {
        self.chain.clone()
    }
}

fn is_transaction_valid(tx: &Transaction, balances: &HashMap<String, i64>) -> bool {
    if tx.from == "COINBASE" {
        return true; // block reward
    }

    if !tx.verify_transaction_signature() {
        return false;
    }

    match balances.get(&tx.from) {
        Some(balance) if *balance >= tx.amount as i64 => true,
        _ => false,
    }
}
