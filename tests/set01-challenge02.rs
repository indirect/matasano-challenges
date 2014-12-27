extern crate serialize;
extern crate xor;

use serialize::hex::{FromHex,ToHex};
use xor::Xor;

#[test]
fn set01_challenge02() {
    let source = "1c0111001f010100061a024b53535009181c";
    let other = "686974207468652062756c6c277320657965";
    let source_bytes = source.from_hex().unwrap();
    let other_bytes = other.from_hex().unwrap();
    let result = source_bytes.xor(other_bytes.as_slice());
    assert_eq!(result.to_hex(), "746865206b696420646f6e277420706c6179");
}
