extern crate rusty_chain;
extern crate clap;

use std::process;

use rusty_chain::blockchain::Blockchain;
use rusty_chain::error::MiningError;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Rusty Chain")
        .version("0.0.1")
        .author("(ↄ) asymmetric")
        .arg(Arg::with_name("difficulty")
             .short("d")
             .long("difficulty")
             .takes_value(true)
             .help("The target difficulty while hashing")
             // TODO take default from somewhere else
             .default_value("5")
             )
         .get_matches();

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
