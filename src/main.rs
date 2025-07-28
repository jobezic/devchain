use crate::blockchain::{Blockchain, Transaction};

mod blockchain;

fn main() {
    let miner_address = "miner123";
    let mut blockchain = Blockchain::new(miner_address);

    blockchain.add_block(Some(Transaction {
        from: "miner123".to_string(),
        to: "Bob".to_string(),
        amount: 5,
    })).unwrap();
    blockchain.add_block(Some(Transaction {
        from: "Bob".to_string(),
        to: "Charlie".to_string(),
        amount: 3,
    })).unwrap();
    blockchain.add_block(Some(Transaction {
        from: "Charlie".to_string(),
        to: "Alice".to_string(),
        amount: 4,
    })).unwrap();

    blockchain.print_chain();

    println!("Blockchain valid? {}", blockchain.is_valid());
    let balances = blockchain.get_balances();
    for (address, balance) in balances {
        println!("{} has {}", address, balance);
    }
}
