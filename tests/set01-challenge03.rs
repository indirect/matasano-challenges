extern crate decrypt;
extern crate "rustc-serialize" as serialize;

use serialize::hex::FromHex;

#[test]
fn set01_challenge03() {
    let source = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes  = source.from_hex().unwrap();
    let answer = decrypt::xor_byte(bytes.as_slice()).unwrap();

    assert_eq!(b'X', answer.key);
    assert_eq!(b"Cooking MC's like a pound of bacon", answer.bytes);
}
