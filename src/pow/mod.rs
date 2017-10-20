use block::{Block, Sha256Hash};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

const TARGET_BITS: u64 = 24;
const MAX_NONCE: i32 = 100_000;

pub struct Pow<'a> {
    target: [u8; 32],
    block: &'a Block,
}

impl<'a> Pow<'a> {
    pub fn new(block: &'a Block) -> Self {
        let mut target = [255; 32];
        target[0] = 0;
        target[1] = 0;
        target[2] = 0;
        Self {
            // TODO clean up
            target: target,
            block: block,
        }
    }

    pub fn run(&self) -> Option<Sha256Hash> {
        let mut nonce = 0i32;

        // hash headers + nonce
        // convert hash to int
        // compare to target
        // if less, return hash
        // otherwise, continue looping
        while nonce < MAX_NONCE {
            let hash = calculate_hash(self.block, nonce);
            // let hash_int = Cursor::new(hash).read_uint::<BigEndian>(32).unwrap();

            if hash < self.target {
                return Some(hash)
            } else {
                nonce += 1;
            }
        }

        None

    }
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
