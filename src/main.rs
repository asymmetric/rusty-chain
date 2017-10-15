extern crate rusty_chain;

fn main() {
    println!("Welcome to Rusty Chain");
    let b = rusty_chain::Block::default();

    println!("b is {:?}", b);
}
