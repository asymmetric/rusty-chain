extern crate rusty_chain;
extern crate clap;

use std::process;

use rusty_chain::blockchain::Blockchain;
use rusty_chain::error::MiningError;

use clap::{Arg, App};

struct RuntimeOptions {
    difficulty: usize,
}

fn main() {
    let default_diff = rusty_chain::DEFAULT_DIFFICULTY.to_string();

    let matches = App::new("Rusty Chain")
        .version("0.0.1")
        .author("(ↄ) asymmetric")
        .arg(Arg::with_name("difficulty")
             .short("d")
             .long("difficulty")
             .takes_value(true)
             .help("The target difficulty while hashing")
             // TODO take default from somewhere else
             .default_value(&default_diff)
             )
         .get_matches();

    println!("Welcome to Rusty Chain");

    let difficulty = matches.value_of("difficulty").unwrap_or(&default_diff).parse::<usize>().unwrap_or(rusty_chain::DEFAULT_DIFFICULTY);

    println!("Creating blockchain with difficulty: {}", difficulty);

    let options = RuntimeOptions {
        difficulty: difficulty,
    };

    run(&options).
        unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1)
        })
}

fn run(options: &RuntimeOptions) -> Result<(), MiningError> {
    let mut chain = Blockchain::new(options.difficulty)?;
    println!("Send 1 RC to foo");
    chain.add_block("enjoy, foo!")?;

    println!("Traversing blockchain:\n");
    chain.traverse();

    Ok(())
}
