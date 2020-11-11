//! Instance-ID
use std::fs::File;
use std::io::{BufRead, BufReader};

use hex;
use sha2::{Digest, Sha256};

use crate::base58;

const BUF_SIZE: usize = 64000;

// Component Header
const HEAD_IID: u8 = 0x30;

/// The Instance-ID is built from the raw data of the media object to be
/// identified and serves as checksum for the media object. The raw data of the
/// media object is split into 64-kB data-chunks. Then we build a hash-tree from
/// those chunks and use the truncated tophash (merkle root) as component body
/// of the Instance-ID.
pub fn instance_id(data_path: &str) -> std::io::Result<(String, String)> {
    let file = File::open(data_path)?;
    let mut reader = BufReader::with_capacity(BUF_SIZE, file);
    Ok(instance_id_reader(&mut reader))
}

pub fn instance_id_reader(reader: &mut dyn BufRead) -> (String, String) {
    let mut leaf_node_digests: Vec<[u8; 32]> = Vec::new();
    while let Ok(chunk) = reader.fill_buf() {
        let n = chunk.len();
        if n == 0 {
            break;
        }
        let zero = &[0];
        let hash = sha256d(&[zero, chunk].concat());
        leaf_node_digests.push(hash);
        reader.consume(n);
    }

    let top_hash_digest = top_hash(&leaf_node_digests);

    let mut instance_id_digest = vec![HEAD_IID];
    instance_id_digest.extend(&top_hash_digest[..8]);

    let code = base58::encode(&instance_id_digest);
    let hex_hash = hex::encode(top_hash_digest);

    (code, hex_hash)
}

pub fn top_hash(hashes: &[[u8; 32]]) -> [u8; 32] {
    if hashes.len() == 1 {
        return hashes[0];
    }

    let mut pairwise_hashed: Vec<[u8; 32]> = Vec::new();
    for hash_pair in hashes.chunks(2) {
        let a = &hash_pair[0];
        let b = hash_pair.get(1).unwrap_or(a);
        pairwise_hashed.push(hash_inner_nodes(a, b));
    }

    top_hash(&pairwise_hashed)
}

fn hash_inner_nodes(a: &[u8], b: &[u8]) -> [u8; 32] {
    let one = &[1];
    sha256d(&[one, a, b].concat())
}

pub fn sha256d(data: &[u8]) -> [u8; 32] {
    let hash1 = Sha256::digest(&data);
    let hash2 = Sha256::digest(&hash1);
    let mut arr: [u8; 32] = Default::default();
    arr.copy_from_slice(hash2.as_ref());
    arr
}
