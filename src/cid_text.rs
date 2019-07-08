use bit_vec::BitVec;
use itertools::Itertools;

use crate::base58;
use crate::hashes::minimum_hash;
use crate::hashes::sliding_window;
use crate::normalization::text_normalize;

const WINDOW_SIZE_CID_T: usize = 13;

// Component Headers
const HEAD_CID_T: u8 = 0x10;
const HEAD_CID_T_PCF: u8 = 0x11;

pub fn content_id_text(text: &str, partial: bool) -> String {
    // 1. Normalize (drop whitespace)
    let text = text_normalize(text, false);

    // 2. Create 13 character n-grams
    let n_grams: Vec<String> = sliding_window(&text, WINDOW_SIZE_CID_T)
        .iter()
        .map(|w| w.chars().intersperse('\u{0020}').collect())
        .collect();

    // 3. Create 32-bit features with xxHash32
    let features: Vec<u32> = n_grams
        .iter()
        .map(|n| xxhash2::hash32(n.as_bytes(), 0))
        .collect();

    // 4. Apply minimum_hash
    let minhash = minimum_hash(features, 64);

    // 5. Collect least significant bits of first 64 minhash signatures
    let lsb: BitVec = minhash.iter().map(|x| (x & 1) == 1).collect();
    let lsb_bytes = lsb.to_bytes();

    // 7. Prepend component header
    let mut content_id_digest = if partial {
        vec![HEAD_CID_T_PCF]
    } else {
        vec![HEAD_CID_T]
    };
    content_id_digest.extend(&lsb_bytes);

    // 8. Encode and return
    base58::encode(&content_id_digest)
}
