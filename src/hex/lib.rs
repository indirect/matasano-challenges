pub fn from_hex(hex_str: &str) -> ~[u8] {
    use std::num;
    use std::str;

    let mut bytes: ~[u8] = ~[];

    for pair in hex_str.as_bytes().chunks(2) {
        let pair_str = str::from_utf8(pair);
        let byte = num::from_str_radix(pair_str, 16).unwrap();
        bytes.push(byte);
    }

    bytes
}

pub fn to_hex(bytes: &[u8]) -> ~str {
    use std::num::ToStrRadix;

    let mut hex = ~"";

    for byte in bytes.iter() {
        hex.push_str(byte.to_str_radix(16));
    }

    hex
}
