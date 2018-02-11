use hex::ToHex;
use chrono::prelude::*;

use error::MiningError;
use pow;
use util;

const HASH_BYTE_SIZE: usize = 32;
pub const HASH_BIT_SIZE: usize = 256;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
pub struct Block {
    // block headers
    timestamp: i64,
    prev_block_hash: Sha256Hash,
    // The nonce is used by every full node to verify the hash.
    nonce: u64,
    // The target this block was mined with.
    difficulty: usize,

    // Instead of transaction, blocks contain data.
    data: Vec<u8>,
}

impl Block {
    // Creates a new block.
    pub fn new(data: &str, difficulty: usize, prev_hash: Sha256Hash) -> Result<Self, MiningError> {
        let mut s = Self {
            timestamp: Self::calculate_timestamp(),
            prev_block_hash: prev_hash,
            nonce: 0,
            difficulty: difficulty,
            data: Self::convert_data(data),
        };

        let started_at = Utc::now();
        s.calculate_hash()
            .ok_or(MiningError::Iteration)
            .and_then(|nonce| {
                println!("Hash/sec: {}", calculate_hashrate(started_at, nonce));

                s.nonce = nonce;

                Ok(s)
            })
    }

    // Creates a genesis block, which is a block with no parent.
    //
    // The `prev_block_hash` field is set to all zeroes.
    pub fn genesis(difficulty: usize) -> Result<Self, MiningError> {
        Self::new("Genesis block", difficulty, Sha256Hash::default())
    }

    // Field getters.
    // Calculates and returns the SHA-256 of the headers.
    pub fn hash(&self) -> Sha256Hash {
        pow::calculate_hash(&self, self.nonce)
    }

    // Returns the hash as a hexadecimal string.
    pub fn pretty_hash(&self) -> String {
        self.hash().to_hex()
    }

    pub fn parent(&self) -> Sha256Hash {
        self.prev_block_hash
    }

    pub fn pretty_parent(&self) -> String {
        self.prev_block_hash.to_hex()
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn pretty_data(&self) -> String {
        String::from_utf8(self.data.clone())
            .unwrap_or_else(|e| format!("Invalid UTF-8 sequence: {}", e))
    }

    fn calculate_hash(&self) -> Option<u64> {
        pow::run(&self)
    }

    // TODO document that tihs only exports a few fields and why
    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend_from_slice(&util::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_block_hash);

        vec
    }

    fn calculate_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    fn convert_data(data: &str) -> Vec<u8> {
        data.to_owned()
            .into()
    }
}

fn calculate_hashrate(started_at: DateTime<Utc>, nonce: u64) -> u64{
    let finished_at = Utc::now();
    let mut d = finished_at.signed_duration_since(started_at).num_seconds();

    if d == 0 {
        d = 1;
    }

    nonce / d as u64
}
