use primitive_types::{U256, U512};

/// A ```BaseGroup``` represents a curve point on the secp256k1 elliptic curve.
/// The curve point has a default value (in hex) of: x = "79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798", y = "483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8"
#[derive(Clone, Debug)]
pub struct BaseGroup {
    pub x: U512,
    pub y: U512,
}
impl Default for BaseGroup {
    fn default() -> Self {
        let x = U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        );
        let y = U256::from_str_radix(
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
            x: U256::from(parsed_x).into(),
            y: U256::from(parsed_y).into(),
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
    pub fn exponentiate_from_256_bit_number(&self, s: U256) -> (U512, U512) {
        let mut updated_x = self.x;
        let mut updated_y = self.y;

        let mut s1 = s;
        let mut s2 = s;
        while s1 > U256::from_dec_str("0").unwrap() {
            updated_x += self.x;
            s1 -= U256::from_dec_str("1").unwrap()
        }
        while s2 > U256::from_dec_str("0").unwrap() {
            updated_y += self.y;
            s2 -= U256::from_dec_str("1").unwrap()
        }
        return (updated_x, updated_y);
    }
    pub fn new() -> Self {
        return Self::default();
    }
}
