use crate::blockchain::Blockchain;

mod blockchain;

fn main() {
    let miner_address = "miner123";
    let mut blockchain = Blockchain::new(4, miner_address);

    blockchain.add_block(Some("Alice pays Bob 5 coins".into()));
    blockchain.add_block(Some("Bob pays Charlie 3 coins".into()));

    blockchain.print_chain();

    println!("Blockchain valid? {}", blockchain.is_valid());
}
