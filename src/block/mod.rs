use sha2::{Digest, Sha256};
use time;

const HASH_SIZE: usize = 32;

#[derive(Debug, Default)]
pub struct Block {
    // block headers
    timestamp: i64,
    prev_block_hash: [u8; HASH_SIZE],
    hash: [u8; HASH_SIZE],

    // transactions
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: &str) -> Self {
        let mut s = Self {
            timestamp: time::now_utc().to_timespec().sec,
            data: data.to_owned().into_bytes(),
            prev_block_hash: Default::default(),
            hash: Default::default(),
        };

        s.hash = s.hash();

        s
    }

    fn hash(&self) -> [u8; HASH_SIZE] {
        let mut hasher = Sha256::default();
        hasher.input(&self.headers());
        let hash = hasher.result();
        println!("the hash is {:?}", hash);

        let mut retval: [u8; HASH_SIZE] = Default::default();

        retval.copy_from_slice(&hash.as_slice());

        retval
    }

    fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.timestamp.to_string().as_bytes());
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);

        vec
    }

}

