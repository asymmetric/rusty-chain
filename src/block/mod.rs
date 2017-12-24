use hex::ToHex;
use time;

use error::MiningError;
use pow;

const HASH_BYTE_SIZE: usize = 32;
pub const HASH_BIT_SIZE: usize = 256;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug, Default)]
pub struct Block {
    // block headers
    timestamp: i64,
    prev_block_hash: Sha256Hash,
    hash: Sha256Hash,
    nonce: u64,

    // transactions
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: &str, prev_hash: Sha256Hash) -> Result<Self, MiningError> {
        let mut s = Self {
            timestamp: Self::timestamp(),
            data: Self::convert_data(data),
            prev_block_hash: prev_hash,
            hash: Default::default(),
            nonce: 0,
        };

        s.calculate_hash()
            .ok_or(MiningError)
            .and_then(|(nonce, hash)| {
                s.nonce = nonce;
                s.hash = hash;

                Ok(s)
            })
    }

    pub fn genesis() -> Result<Self, MiningError> {
        Self::new("Genesis block", Sha256Hash::default())
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
        String::from_utf8(self.data.clone())
            .unwrap_or_else(|e| format!("Invalid UTF-8 sequence: {}", e))
    }

    fn calculate_hash(&self) -> Option<(u64, Sha256Hash)> {
        pow::run(&self)
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&self.timestamp
            .to_string()
            .as_bytes());
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);

        vec
    }

    fn timestamp() -> i64 {
        time::now_utc()
            .to_timespec()
            .sec
    }

    fn convert_data(data: &str) -> Vec<u8> {
        data.to_owned()
            .into_bytes()
    }
}
