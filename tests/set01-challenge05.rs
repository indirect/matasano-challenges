extern crate "rustc-serialize" as serialize;
extern crate xor;

use xor::Xor;
use serialize::hex::FromHex;

#[test]
fn set01_challenge05() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let ciphered_bytes = plaintext.as_bytes().xor(key);
    let expected_bytes = concat!(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a2622632427276527",
        "2a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    ).from_hex().unwrap();

    assert_eq!(expected_bytes, ciphered_bytes);
}