extern crate rusty_chain;

use std::process;

use rusty_chain::blockchain::Blockchain;
use rusty_chain::error::MiningError;

fn main() {
    println!("Welcome to Rusty Chain");

    run().
        unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1)
        })
}

fn run() -> Result<(), MiningError> {
    let mut chain = Blockchain::new()?;
    println!("Send 1 RC to foo");
    chain.add_block("enjoy, foo!")?;

    println!("Traversing blockchain:\n");
    chain.traverse();

    Ok(())
}
