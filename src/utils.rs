use sha2::{Digest, Sha256};

use crate::Block;

pub(crate) fn hash_block(block: &Block) -> String {
    let b = serde_json::to_string(block).expect("Error");
    hash_string_sha256(&b)
}

pub fn hash_string_sha256(string: &str) -> String {
    let mut buf = [0u8; 64];
    let hash = Sha256::new()
        .chain_update(string.as_bytes())
        .finalize();
    base16ct::lower::encode_str(&hash, &mut buf).unwrap().to_string()
}
