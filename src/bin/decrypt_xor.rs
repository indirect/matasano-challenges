extern crate serialize;
extern crate xor;

use serialize::hex::{FromHex,ToHex};
use xor::Xor;

fn main() {
    let args   = std::os::args();
    let key    = args[1].as_bytes();
    let cipher = args[2].from_hex().unwrap();
    let answer = cipher.xor(key);

    println!("ciphertext: {}", cipher.to_hex());
    println!("key:        {} ", key.to_hex());
    println!("plaintext:  {}", String::from_utf8(answer).unwrap());
}
