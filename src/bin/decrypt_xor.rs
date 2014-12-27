extern crate serialize;
extern crate xor;

use serialize::hex::{FromHex,ToHex};
use xor::Xor;

fn main() {
    let args = std::os::args();
    let key;
    let cipher;
    let text;

    key = args[1].to_string();
    cipher = args[2].from_hex().unwrap();
    let plain = cipher.xor(key.as_bytes());
    text = String::from_utf8(plain).unwrap();

    println!("ciphertext: {}", cipher.to_hex());
    println!("key:        {} {}", key.as_bytes().to_hex(), key.to_ascii());
    println!("plaintext:  {}", text);
}
