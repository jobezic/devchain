use crate::blockchain::Blockchain;

mod blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block data".into());
    blockchain.add_block("Second block data".into());
    blockchain.add_block("Third block data".into());

    blockchain.print_chain();

    println!("Blockchain valid? {}", blockchain.is_valid());
}
