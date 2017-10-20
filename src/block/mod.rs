use hex::ToHex;
use time;
use pow::Pow;

const HASH_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_SIZE];

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
            timestamp: timestamp(),
            data: convert_data(data),
            prev_block_hash: prev_hash,
            hash: Default::default(),
        };

        s.hash = s.calculate_hash();

        s
    }

    pub fn genesis() -> Self {
        Self { timestamp: timestamp(), data: convert_data("Genesis block"), ..Default::default() }
    }

    pub fn hash(&self) -> Sha256Hash {
        self.hash.clone()
    }

    pub fn pretty_hash(&self) -> String {
        self.hash.to_hex()
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn pretty_data(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(data) => data,
            Err(e) => format!("Invalid UTF-8 sequence: {}", e),
        }
    }

    fn calculate_hash(&self) -> Sha256Hash {
        let pow = Pow::new(&self);
        let hash = match pow.run() {
            Some(hash) => hash,
            // TODO return an Option instead
            None => [0; 32],
        };

        hash
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.timestamp.to_string().as_bytes());
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);

        vec
    }

}

fn timestamp() -> i64 {
    time::now_utc().to_timespec().sec
}

fn convert_data(data: &str) -> Vec<u8> {
    data.to_owned().into_bytes()
}
