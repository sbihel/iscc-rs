use crate::base58;
use crate::hashes::similarity_hash;

const HEAD_CID_M: u8 = 0x18;
const HEAD_CID_M_PCF: u8 = 0x19;

pub fn content_id_mixed(cids: &[&str], partial: bool) -> String {
    // 1. Decode CIDs
    let decoded: Vec<Vec<u8>> = cids.iter().map(|cid| base58::decode(cid)).collect();

    // 2. Extract first 8-bytes
    let mut array = [0; 8];
    let mut truncated: Vec<u64> = Vec::new();
    for code in decoded {
        array.copy_from_slice(&code[..8]);
        truncated.push(u64::from_be_bytes(array));
    }

    // 3. Apply Similarity hash
    let simhash_digest = similarity_hash(truncated);

    // 4. Prepend the 1-byte component header
    let mut content_id_mixed = if partial {
        vec![HEAD_CID_M_PCF]
    } else {
        vec![HEAD_CID_M]
    };
    content_id_mixed.extend(&simhash_digest);

    // 5. Encode and return
    base58::encode(&content_id_mixed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{content_id_image, content_id_text};

    #[test]
    fn test_content_id_mixed() {
        let cid_t_1 = content_id_text("Some Text", false);
        let cid_t_2 = content_id_text("Another Text", false);

        let cid_m = content_id_mixed(&[&cid_t_1], false);
        assert_eq!(cid_m, "CM3k9pp7JS7nP".to_string());

        let cid_m = content_id_mixed(&[&cid_t_1, &cid_t_2], false);
        assert_eq!(cid_m, "CM3kHkNRGvnhB".to_string());

        let cid_i = content_id_image("test_data/lenna.jpg", false).unwrap();
        let cid_m = content_id_mixed(&[&cid_t_1, &cid_t_2, &cid_i], false);
        assert_eq!(cid_m, "CM3hswzATv9d3".to_string());
    }
}
