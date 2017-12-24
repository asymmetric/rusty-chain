use block::{HASH_BIT_SIZE, Block, Sha256Hash};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;

// How many leading zeroes the 32 bit hash should have in order to be considered valid.
const TARGET_BITS: usize = 5;
const MAX_NONCE: u64 = 100_000;

// Runs the POW algorithm on a block, i.e. it calculates HASH(header + nonce) until the hash
// fulfills the difficulty requirement.
pub fn run(block: &Block) -> Option<(u64, Sha256Hash)> {
    let mut target = BigUint::one();
    // The target is a number we compare the hash to. It is a HASH_BIT_SIZE bit binary with DIFFICULTY_BITS
    // zeroes.
    target = target << (HASH_BIT_SIZE - TARGET_BITS);

    for nonce in 0..MAX_NONCE {
        let hash = calculate_hash(&block, nonce);
        let hash_int = BigUint::from_bytes_be(&hash);

        if hash_int < target {
            return Some((nonce, hash));
        }
    }

    None

}

fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
    let mut headers = block.headers();
    headers.push(nonce as u8);
    headers.push(TARGET_BITS as u8);

    let mut hasher = Sha256::new();
    hasher.input(&headers);
    let mut hash = Sha256Hash::default();

    hasher.result(&mut hash);

    hash
}
