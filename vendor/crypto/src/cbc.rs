extern crate xor;

use std::slice::SliceExt;
use ecb;
use self::xor::Xor;

pub fn decrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
    // we can only decrypt ciphertext composed of 16 byte blocks
    assert_eq!(0, bytes.len() % 16);

    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut previous_block = iv;

    for block in bytes.chunks(16) {
        let block_decrypt = ecb::decrypt(key, block.as_slice());
        let block_result = block_decrypt.xor(previous_block.as_slice());
        result.push_all(block_result.as_slice());
        previous_block = block;
    }

    result
}

#[test]
fn test_decrypt_xors_each_block() {
    let key = b"YELLOW SUBMARINE";
    let iv: Vec<u8> = ::std::iter::repeat(1).take(16).collect();
    let bytes = vec![
        65, 49, 236, 68, 145, 144, 12, 170, 200, 25, 222, 96, 105, 172, 73, 169,
        51, 59, 76, 80, 157, 114, 74, 222, 112, 79, 7, 231, 91, 171, 45, 171
    ];
    let result = decrypt(key.as_slice(), iv.as_slice(), bytes.as_slice());

    let plain = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
    ];
    assert_eq!(plain, result);
}

pub fn decrypt_zero_iv(key: &[u8], bytes: &[u8]) -> Vec<u8> {
    let iv: Vec<u8> = ::std::iter::repeat(0).take(bytes.len()).collect();
    decrypt(key, iv.as_slice(), bytes)
}

#[test]
fn test_decrypt_zero_iv() {
    let bytes = vec![
        221, 100, 7, 233, 139, 64, 190, 180, 199, 215, 195, 15, 236, 63, 10, 19,
        34, 188, 184, 162, 120, 186, 77, 181, 64, 13, 161, 30, 102, 161, 68, 23
    ];
    let plain = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
    ];
    assert_eq!(plain, decrypt_zero_iv(b"YELLOW SUBMARINE".as_slice(), bytes.as_slice()));
}

pub fn encrypt(key: &[u8], iv: &[u8], bytes: &[u8]) -> Vec<u8> {
    assert_eq!(0, bytes.len() % 16);
    assert_eq!(16, iv.len());

    let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut block_result = iv.to_vec();

    for block in bytes.chunks(16) {
        let block_input = block.xor(block_result.as_slice());
        block_result = ecb::encrypt(key, block_input.as_slice());
        result.push_all(block_result.as_slice());
    }

    result
}

#[test]
fn test_encrypt_xors_each_block() {
    let key = b"YELLOW SUBMARINE";
    let iv: Vec<u8> = ::std::iter::repeat(1).take(16).collect();
    let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let ecb_xor_one = ecb::encrypt(key, bytes.slice_to(16).xor(iv.as_slice()).as_slice());
    let ecb_xor_two = ecb::encrypt(key, ecb_xor_one.xor(bytes.slice_from(16)).as_slice());
    let cbc_result = encrypt(b"YELLOW SUBMARINE", iv.as_slice(), bytes.as_slice());

    assert_eq!(32, cbc_result.len());
    assert_eq!(ecb_xor_one.slice_to(16), cbc_result.slice_to(16));
    assert_eq!(ecb_xor_two.slice_to(16), cbc_result.slice_from(16));
}

pub fn encrypt_zero_iv(key: &[u8], bytes: &[u8]) -> Vec<u8> {
    let iv: Vec<u8> = ::std::iter::repeat(0).take(16).collect();
    encrypt(key, iv.as_slice(), bytes)
}

#[test]
fn test_encrypt_zero_iv() {
    let key = b"YELLOW SUBMARINE";
    let iv: Vec<u8> = ::std::iter::repeat(0).take(16).collect();
    let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    assert_eq!(
        encrypt(key.as_slice(), iv.as_slice(), bytes.as_slice()),
        encrypt_zero_iv(key.as_slice(), bytes.as_slice())
    );
}