use block::Block;
use error::MiningError;

pub struct Blockchain {
    // The target this block was mined with.
    difficulty: usize,
    blocks: Vec<Block>,
}

impl Blockchain {
    // Initializes a new blockchain with a genesis block.
    pub fn new() -> Result<Self, MiningError> {
        let blocks = Block::genesis()?;

        Ok(Self { blocks: vec![blocks] })
    }

    // Adds a newly-forged block to the chain.
    pub fn add_block(&mut self, data: &str) -> Result<(), MiningError> {
        let block: Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    block = Block::new(data, prev.hash())?;
                }
                // Adding a block to an empty blockchain is an error, a genesis block needs to be
                // created first.
                None => {
                    return Err(MiningError::NoParent)
                }
            }
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("hash: {:?}", block.pretty_hash());
            println!("parent: {:?}", block.pretty_parent());
            println!("data: {:?}", block.pretty_data());
            println!()
        }
    }
}
