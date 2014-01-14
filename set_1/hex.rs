extern mod extra;

fn from_hex(hex_str: &str) -> ~[u8] {
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

fn to_hex(bytes: &[u8]) -> ~str {
    use std::num::ToStrRadix;

    let mut hex = ~"";

    for byte in bytes.iter() {
        hex.push_str(byte.to_str_radix(16));
    }

    hex
}

fn main() {
    use extra::base64::{ToBase64, FromBase64, STANDARD};

    let hex_str = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_bytes = from_hex(hex_str);
    println!("{}", std::str::from_utf8(hex_bytes));
    
    println!("{}", to_hex(hex_bytes));
    
    println!("{}", hex_bytes.to_base64(STANDARD));
}
