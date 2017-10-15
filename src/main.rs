extern crate rusty_chain;

use rusty_chain::block::Block;

fn main() {
    println!("Welcome to Rusty Chain");
    let b = Block::default();

    println!("b is {:?}", b);
}
