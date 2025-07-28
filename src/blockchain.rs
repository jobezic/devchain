use sha2::{Digest, Sha256};
use chrono::Utc;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let content = format!("{}{}{}{}", index, &timestamp, &data, &previous_hash);
        let hash = calculate_hash(&content);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
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
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, "Genesis Block".into(), "0".into());
        Blockchain { chain: vec![genesis] }
    }

    pub fn add_block(&mut self, data: String) {
        let last = self.chain.last().unwrap();
        let new_block = Block::new(last.index + 1, data, last.hash.clone());
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let content = format!("{}{}{}{}", current.index, current.timestamp, current.data, current.previous_hash);
            if current.hash != calculate_hash(&content) {
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
