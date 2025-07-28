use sha2::{Digest, Sha256};
use chrono::Utc;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash;

        let start = Instant::now();

        loop {
            let content = format!("{}{}{}{}{}", index, &timestamp, &data, &previous_hash, nonce);
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
            data,
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
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::new(0, "Genesis Block".into(), "0".into(), difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last = self.chain.last().unwrap();
        let new_block = Block::new(
            last.index + 1,
            data,
            last.hash.clone(),
            self.difficulty,
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
                "{}{}{}{}{}",
                current.index,
                current.timestamp,
                current.data,
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
