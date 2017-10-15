extern crate rusty_chain;

use rusty_chain::blockchain::Blockchain;

fn main() {
    println!("Welcome to Rusty Chain");

    let mut chain = Blockchain::new();
    println!("Send 1 RC to foo");
    chain.add_block("enjoy, foo!");

    chain.traverse();
}
