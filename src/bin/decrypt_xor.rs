extern crate decrypt;
extern crate serialize;
extern crate xor;

use serialize::hex::{FromHex,ToHex};
use xor::Xor;

fn main() {
    let args = std::os::args();

    if args.len() < 3 {
        let cipher = args[1].from_hex().unwrap();
        let answer = decrypt::xor_repeating(cipher.as_slice()).unwrap();

        println!("ciphertext: {}", cipher.to_hex());
        println!("key:        {} {}", answer.key.to_hex(), String::from_utf8_lossy(answer.key.as_slice()));
        println!("plaintext:  {}", String::from_utf8_lossy(answer.bytes.as_slice()));
    } else {
        let key = args[1].as_bytes();
        let cipher = args[2].from_hex().unwrap();
        let plain = cipher.xor(key);

        println!("ciphertext: {}", cipher.to_hex());
        println!("key:        {} {}", key.to_hex(), key.to_ascii());
        println!("plaintext:  {}", String::from_utf8_lossy(plain.as_slice()));
    }
}
