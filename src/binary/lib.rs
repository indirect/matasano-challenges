#[crate_id = "binary#0.1"];
#[crate_type = "lib"];

pub trait Binary {
    fn to_binary(&self) -> ~str;
}

impl<'a> Binary for &'a [u8] {
    fn to_binary(&self) -> ~str {
        use std::num::ToStrRadix;

        let mut binary = ~"";

        for byte in self.iter() {
            binary.push_str(format!("{:0>8s}", byte.to_str_radix(2)));
        }

        binary
    }
}
