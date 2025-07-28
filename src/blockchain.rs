use sha2::{Digest, Sha256};
use chrono::Utc;
use std::time::Instant;

const BLOCK_REWARD: u64 = 50; // Reward per block in "coins"
const DIFFICULTY: usize = 4; // Mining difficulty

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    transactions: Vec<String>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, miner_address: &str, previous_hash: String, difficulty: usize, data: Option<String>) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash;

        // Create the reward transaction (coinbase)
        let mut transactions = vec![format!("Reward {} coins to {}", BLOCK_REWARD, miner_address)];
        if let Some(extra_data) = data {
            transactions.push(extra_data);
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
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    reward_address: String,
}

impl Blockchain {
    pub fn new(miner_address: &str) -> Self {
        let genesis = Block::new(0, miner_address, "0".into(), DIFFICULTY, Some("Genesis block".into()));
        Blockchain {
            chain: vec![genesis],
            difficulty: DIFFICULTY,
            reward_address: miner_address.to_string(),
        }
    }

    pub fn add_block(&mut self, data: Option<String>) {
        let last = self.chain.last().unwrap();
        let new_block = Block::new(
            last.index + 1,
            &self.reward_address,
            last.hash.clone(),
            self.difficulty,
            data,
        );
        self.chain.push(new_block);
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

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}
