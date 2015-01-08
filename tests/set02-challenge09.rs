extern crate crypto;
use crypto::pkcs7;

#[test]
fn set02_challenge09() {
    let padded_submarine = pkcs7::pad( "YELLOW SUBMARINE".as_bytes(), 20);
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes(), padded_submarine);
}