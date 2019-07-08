use crate::base58::encode;
use crate::hashes::{similarity_hash, sliding_window};
use crate::normalization::text_normalize;

const WINDOW_SIZE_MID: usize = 4;
const HEAD_MID: u8 = 0x00;
const INPUT_TRIM: usize = 128;

pub fn meta_id(title: &str, extra: &str) -> (String, String, String) {
    // 1. Normalization
    let title_norm = text_normalize(title, true);
    let extra_norm = text_normalize(extra, true);

    // 2. Trimming
    let title_trimmed = text_trim(&title_norm);
    let extra_trimmed = text_trim(&extra_norm);

    // 3. Concatenate
    let concat = vec![title_trimmed.clone(), extra_trimmed.clone()].join(" ");

    // 4. Create a list of n-grams
    let n_grams = sliding_window(&concat, WINDOW_SIZE_MID);

    // 5. Encode n-grams and create xxhash64-digest
    let hash_digests: Vec<u64> = n_grams
        .iter()
        .map(|n| xxhash2::hash64(n.as_bytes(), 0))
        .collect();

    // 6. Apply similarity_hash
    let simhash_digest = similarity_hash(hash_digests);

    // 7. Prepend header-byte
    let mut meta_id_digest = vec![HEAD_MID];
    meta_id_digest.extend(simhash_digest);

    // 8. Encode with base58_iscc
    let meta_id = encode(&meta_id_digest);
    (meta_id, title_trimmed, extra_trimmed)
}

/// Trim text such that its UTF-8 encoded byte representation does not exceed
/// 128-bytes each. Remove leading and trailing whitespace.
pub fn text_trim(text: &str) -> String {
    let input_trim = text.len().min(INPUT_TRIM);
    String::from_utf8_lossy(&text.as_bytes()[..input_trim])
        .replace("�", "")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_text() {
        let multibyte_2 = "ü".repeat(128);
        let trimmed = text_trim(&multibyte_2);
        assert_eq!(trimmed.chars().count(), 64);
        assert_eq!(trimmed.len(), 128);
        let multibyte_3 = "驩".repeat(128);
        let trimmed2 = text_trim(&multibyte_3);
        assert_eq!(trimmed2.chars().count(), 42);
        assert_eq!(trimmed2.len(), 126);
        let mixed = "Iñtërnâtiônàlizætiøn☃💩".repeat(6);
        let trimmed3 = text_trim(&mixed);
        assert_eq!(trimmed3.chars().count(), 85);
        assert_eq!(trimmed3.len(), 128);
    }
}
