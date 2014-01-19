extern mod extra;
extern mod xor;

use extra::hex::{FromHex, ToHex};
use xor::Xor;

#[test]
fn test_xor() {
    let source = ~"1c0111001f010100061a024b53535009181c";
    let other = ~"686974207468652062756c6c277320657965";
    let result = source.from_hex().unwrap().xor(other.from_hex().unwrap());
    assert_eq!(result.to_hex(), ~"746865206b696420646f6e277420706c6179");
}
