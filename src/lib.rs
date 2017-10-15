extern crate sha2;
extern crate time;

use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
pub struct Block {
    // block headers
    timestamp: i64,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,

    // transactions
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>) -> Self {
        let mut s = Self {
            timestamp: time::now_utc().to_timespec().sec,
            data: data,
            prev_block_hash: Vec::new(),
            hash: Vec::new(),
        };

        s.hash = s.hash();

        s
    }

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::default();
        hasher.input(&self.headers());
        let hash = hasher.result();
        println!("the hash is {:?}", hash);

        hash.to_vec()
    }

    fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.timestamp.to_string().as_bytes());
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);

        vec
    }

}
