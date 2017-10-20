use block::{HASH_BIT_SIZE, Block, Sha256Hash};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use num_bigint::BigUint;
use num_traits::One;

const TARGET_BITS: usize = 5;
const MAX_NONCE: i32 = 100_000;

pub struct Pow<'a> {
    target: BigUint,
    block: &'a Block,
}

impl<'a> Pow<'a> {
    pub fn new(block: &'a Block) -> Self {
        let mut target = BigUint::one();
        target = target << (HASH_BIT_SIZE - TARGET_BITS);

        Self {
            target: target,
            block: block,
        }
    }

    pub fn run(&self) -> Option<Sha256Hash> {
        let mut nonce = 0i32;

        while nonce < MAX_NONCE {
            let hash = Self::calculate_hash(self.block, nonce);
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < self.target {
                return Some(hash)
            } else {
                nonce += 1;
            }
        }

        None

    }

    fn calculate_hash(block: &Block, nonce: i32) -> Sha256Hash {
        let mut headers = block.headers();
        headers.push(nonce as u8);

        let mut hasher = Sha256::new();
        hasher.input(&headers);
        let mut hash = Sha256Hash::default();

        hasher.result(&mut hash);

        hash
    }
}
