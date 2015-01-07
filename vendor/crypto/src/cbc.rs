use std::slice::SliceExt;
use {ecb, pkcs7};

pub fn decrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
    // we can only decrypt ciphertext composed of 16 byte blocks
    assert_eq!(0, bytes.len() % 16);
    let mut last_block = iv;
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());

    for (index, block) in bytes.chunks(16).enumerate() {
        println!("block number {}: {}", index, block);

    }

    vec![]
}

pub fn decrypt_zero_iv(key: &[u8], bytes: &[u8]) -> Vec<u8> {
    let iv: Vec<u8> = ::std::iter::repeat(0).take(bytes.len()).collect();
    decrypt(key, iv.as_slice(), bytes)
}

pub fn encrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut last_block = iv;

    for (index, chunk) in bytes.chunks(16).enumerate() {
        if chunk.len() < 16 {
            let block = pkcs7::pad(chunk, 20);
            println!("block number {}: {}", index, block);
        } else {
            println!("block number {}: {}", index, chunk);
        }
    }

    // let combined_bytes = iv.to_vec().xor(bytes);
    ecb::encrypt_zero_iv(key, bytes)
}

#[test]
fn cbc_xors_first_block() {
    let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let iv: Vec<u8> = ::std::iter::repeat(0).take(16).collect();
    let cbc_bytes = encrypt("YELLOW SUBMARINE".as_bytes(), iv.as_slice(), bytes.as_slice());
    assert_eq!(cbc_bytes, b"foo");
}

pub fn encrypt_zero_iv(key: &[u8], bytes: &[u8]) -> Vec<u8> {
    let iv: Vec<u8> = ::std::iter::repeat(0).take(bytes.len()).collect();
    encrypt(key, iv.as_slice(), bytes)
}
