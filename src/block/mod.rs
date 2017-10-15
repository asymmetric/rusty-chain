use sha2::{Digest, Sha256};
use time;

const HASH_SIZE: usize = 32;

type Sha256Hash = [u8; HASH_SIZE];

#[derive(Debug, Default)]
pub struct Block {
    // block headers
    timestamp: i64,
    prev_block_hash: Sha256Hash,
    hash:  Sha256Hash,

    // transactions
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: &str, prev_hash: Sha256Hash) -> Self {
        let mut s = Self {
            timestamp: time::now_utc().to_timespec().sec,
            data: data.to_owned().into_bytes(),
            prev_block_hash: prev_hash,
            hash: Default::default(),
        };

        s.hash = s.hash();

        s
    }

    fn hash(&self) -> Sha256Hash {
        let mut hasher = Sha256::default();
        hasher.input(&self.headers());
        let hash = hasher.result();
        println!("the hash is {:?}", hash);

        let mut retval: Sha256Hash = Default::default();

        retval.copy_from_slice(&hash.as_slice());

        retval
    }

    // TODO no need to use a Vec here
    fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.timestamp.to_string().as_bytes());
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);

        vec
    }

}

