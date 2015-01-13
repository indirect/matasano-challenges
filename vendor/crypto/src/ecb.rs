extern crate openssl;
use self::openssl::crypto::symm;

fn crypt(mode: symm::Mode, key: &[u8], data: &[u8]) -> Vec<u8> {
    // ecb doesn't actually use an iv, but it's part of the API
    let iv: Vec<u8> = Vec::new();

    let crypter = symm::Crypter::new(symm::Type::AES_128_ECB);
    crypter.init(mode, key, iv);
    crypter.pad(false);

    let result = crypter.update(data);
    crypter.finalize();
    result
}

pub fn decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    crypt(symm::Mode::Decrypt, key, data)
}

#[test]
fn test_decrypt_with_bytes() {
    let cipher = vec![209, 170, 79, 101, 120, 146, 101, 66, 251, 182, 221, 135, 108, 210, 5, 8];
    let bytes = decrypt(b"YELLOW SUBMARINE", cipher.as_slice());
    assert_eq!(b"YELLOW SUBMARINE", bytes);
}

pub fn encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    assert_eq!(0, data.len() % 16);
    crypt(symm::Mode::Encrypt, key, data)
}

#[test]
fn test_encrypt_with_exact_block_size() {
    let plain = b"YELLOW SUBMARINE";
    let bytes = encrypt(b"YELLOW SUBMARINE", plain);
    assert_eq!(16, bytes.len());
    assert_eq!(
        vec![209, 170, 79, 101, 120, 146, 101, 66, 251, 182, 221, 135, 108, 210, 5, 8],
        bytes
    );
}


pub fn detect(bytes: &[u8]) -> bool {
    super::repeated_block_count(bytes, 16) > 0
}

#[test]
fn is_ecb_with_repeated_16_bytes() {
    let bytes = vec![
        1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,
        1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16
    ];
    assert_eq!(true, detect(bytes.as_slice()));
}

#[test]
fn is_not_ecb_with_repeated_15_bytes() {
    let bytes = vec![
        1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
        1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
    ];
    assert_eq!(false, detect(bytes.as_slice()));
}
