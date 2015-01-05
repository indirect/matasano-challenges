extern crate crypto;

#[test]
fn set02_challenge09() {
    let padded_submarine = crypto::pkcs7(20, "YELLOW SUBMARINE".as_bytes());
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes(), padded_submarine);
}