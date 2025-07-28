use crate::blockchain::{Blockchain, Transaction};

mod blockchain;

fn main() {
    let miner_address = "miner123";
    let mut blockchain = Blockchain::new(miner_address);

    blockchain.add_block(Some(Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 5,
    }));
    blockchain.add_block(Some(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 3,
    }));

    blockchain.print_chain();

    println!("Blockchain valid? {}", blockchain.is_valid());
}
