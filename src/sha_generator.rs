use crate::group_utils;
use primitive_types::U256;
use sha2::{Digest, Sha256};

fn SHA_PRIME() -> U256 {
    let result: i32 = 2 ^ 256 - 2 ^ 32 - 2 ^ 9 - 2 ^ 8 - 2 ^ 7 - 2 ^ 6 - 2 ^ 4 - 1;
    let conv = U256::from_str_radix(result.to_string().as_str(), 10);
    let SHA_PRIME = match conv {
        Ok(conv) => conv,
        Err(e) => panic!("Could not parse SHA_PRIME: {}", e),
    };
    return SHA_PRIME;
}

pub fn sha_generate_address() {
    let G = group_utils::BaseGroup::new();
    let summation_result = G.exponentiate_from_256_bit_number(SHA_PRIME());
    println!("{:?} ", summation_result);
    let mut hasher = Sha256::new();
    hasher.update(summation_result.0.to_string().as_bytes());
    let result = hasher.finalize();
    println!("{:?} ", result);
}
