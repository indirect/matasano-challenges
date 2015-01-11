extern crate "rustc-serialize" as serialize;
extern crate xor;

use serialize::hex::{FromHex,ToHex};
use xor::Xor;

fn main() {
    let source = "1c0111001f010100061a024b53535009181c";
    println!("Hex:    {:?}", source);
    let other = "686974207468652062756c6c277320657965";
    println!("Xor:    {:?}", other);
    let result = source.from_hex().unwrap().xor(other.from_hex().unwrap().as_slice());
    println!("Result: {:?}", result.to_hex());
    println!("Text:   {:?}", String::from_utf8(result).unwrap());
}
