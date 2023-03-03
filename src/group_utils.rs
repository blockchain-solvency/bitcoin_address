use primitive_types::{U256, U512};

fn SHA_PRIME() -> U256 {
    let result: i32 = 2 ^ 256 - 2 ^ 32 - 2 ^ 9 - 2 ^ 8 - 2 ^ 7 - 2 ^ 6 - 2 ^ 4 - 1;
    let conv = U256::from_str_radix(result.to_string().as_str(), 10);
    let SHA_PRIME = match conv {
        Ok(conv) => conv,
        Err(e) => panic!("Could not parse SHA_PRIME: {}", e),
    };
    return SHA_PRIME;
}
/// A ```BaseGroup``` represents a curve point on the secp256k1 elliptic curve.
/// The curve point has a default value (in hex) of: x = "79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798", y = "483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8"
#[derive(Clone, Debug)]
pub struct BaseGroup {
    pub x: U512,
    pub y: U512,
}
impl Default for BaseGroup {
    fn default() -> Self {
        let x = U512::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        );
        let y = U512::from_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        );
        let parsed_x = match x {
            Ok(parsed_value) => parsed_value,
            Err(err) => panic!("Could not parse value: {}", err),
        };
        let parsed_y = match y {
            Ok(parsed_value) => parsed_value,
            Err(err) => panic!("Could not parse value: {}", err),
        };
        return Self {
            x: parsed_x,
            y: parsed_y,
        };
    }
}
impl BaseGroup {
    pub fn exponentiate(&self, s: u64) -> (U512, U512) {
        let mut updated_x = self.x;
        let mut updated_y = self.y;

        let mut s1 = s;
        let mut s2 = s;
        while s1 > 0 {
            updated_x += self.x;
            s1 -= 1
        }
        while s2 > 0 {
            updated_y += self.y;
            s2 -= 1
        }
        return (updated_x, updated_y);
    }
    pub fn new() -> Self {
        return Self::default();
    }
}
