#![allow(unstable)]
extern crate "rustc-serialize" as serialize;
extern crate xor;

use serialize::hex::ToHex;
use xor::Xor;

fn main() {
    let args   = std::os::args();
    let key    = args[1].as_bytes();
    let plain  = args[2].as_bytes();
    let cipher = plain.xor(key);

    println!("plaintext:  {:?}", plain.to_hex());
    println!("key:        {:?} {:?}", key.to_hex(), String::from_utf8_lossy(key.as_slice()));
    println!("ciphertext: {:?}", cipher.to_hex());
}
