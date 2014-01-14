extern mod extra;
use extra::hex::{FromHex, ToHex};

pub trait Xor {
    fn xor(&self, other: &[u8]) -> ~[u8];
}

impl<'a> Xor for &'a [u8] {
    fn xor(&self, other: &[u8]) -> ~[u8] {
        self.iter().zip(other.iter())
            .map(|(a, b)| { a.bitxor(b) })
            .collect()
    }
}

fn main() {
    let source = ~"1c0111001f010100061a024b53535009181c";
    let other = ~"686974207468652062756c6c277320657965";
    let result = source.from_hex().unwrap().xor(other.from_hex().unwrap());
    println!("{}", result.to_hex());
}

#[test]
fn test_xor() {
    let source = ~"1c0111001f010100061a024b53535009181c";
    let other = ~"686974207468652062756c6c277320657965";
    let result = source.from_hex().unwrap().xor(other.from_hex().unwrap());
    assert_eq!(result.to_hex(), ~"746865206b696420646f6e277420706c6179");
}

// rustc --test problem_2.rs && ./problem_2