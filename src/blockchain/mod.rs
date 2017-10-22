use block::Block;
use error::MiningError;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Result<Self, MiningError> {
        let blocks = Block::genesis()?;

        Ok(Self { blocks: vec![blocks] })
    }

    pub fn add_block(&mut self, data: &str) -> Result<bool, MiningError> {
        let block: Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    block = Block::new(data, prev.hash())?;
                }
                None => return Ok(false),
            }
        }

        self.blocks.push(block);

        Ok(true)
    }

    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("hash: {:?}", block.pretty_hash());
            println!("data: {:?}", block.pretty_data());
            println!()
        }
    }
}
