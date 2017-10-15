use block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: &str) -> bool {
        let block: Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    block = Block::new(data, prev.hash());
                }
                None => return false
            }
        }

        self.blocks.push(block);

        true
    }

    pub fn traverse(&self) {
        for block in self.blocks.iter() {
            println!("block has hash {:?}", block.hash());
        }
    }
}
