#![allow(unstable)]
pub mod ecb;
pub mod cbc;
pub mod pkcs7;

#[derive(Copy)]
pub enum Cipher {
    ECB,
    CBC
}

impl std::fmt::Show for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            &Cipher::ECB => write!(f, "ECB"),
            &Cipher::CBC => write!(f, "CBC")
        }
    }
}

pub fn oracle(data: &[u8]) -> Cipher {
    match ecb::detect(data) {
        true => Cipher::ECB,
        false => Cipher::CBC
    }
}

#[test]
fn test_oracle_with_ecb() {
    let mut plain: Vec<u8> = Vec::with_capacity(256);
    for b in range(0, 240) { plain.push(b % 16); }
    let cipher = ecb::encrypt(b"YELLOW SUBMARINE".as_slice(), plain.as_slice());
    match oracle(cipher.as_slice()) {
        Cipher::ECB => (),
        Cipher::CBC => panic!("Oh no! The oracle is wrong")
    };
}

#[test]
fn test_oracle_with_cbc() {
    let mut plain: Vec<u8> = Vec::with_capacity(256);
    for b in range(0, 240) { plain.push(b % 16); }
    let cipher = cbc::encrypt_zero_iv(b"YELLOW SUBMARINE".as_slice(), plain.as_slice());
    match oracle(cipher.as_slice()) {
        Cipher::ECB => panic!("Oh no! The oracle is wrong"),
        Cipher::CBC => ()
    };
}
