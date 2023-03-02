use primitive_types::U512;


#[derive(Clone, Debug)]
pub struct BaseGroup {
    pub x: U512,
    pub y: U512,
}
impl Default for BaseGroup {
    fn default() -> Self {
        let x = U512::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16);
        let y = U512::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16);
        let parsed_x = match x{
            Ok(parsed_value) => parsed_value,
            Err(err) => panic!("Could not parse value: {}", err)
        };
        let parsed_y = match y{
            Ok(parsed_value) => parsed_value,
            Err(err) => panic!("Could not parse value: {}", err)
        };
        return Self{
            x:parsed_x,
            y:parsed_y
        }
    }
}
impl BaseGroup {
    pub fn exponentiate(&self, s:u64) -> (U512, U512) {
        let mut updated_x = self.x;
        let mut updated_y = self.y;
         
        let mut s1 = s;
        let mut s2 = s;
        while s1>0{
            updated_x+=self.x;
            s1-=1
        }
        while s2>0{
            updated_y+=self.y;
            s2-=1
        }
        return (updated_x, updated_y);
    }
    pub fn new() -> Self{
        return Self::default();
    }
}

