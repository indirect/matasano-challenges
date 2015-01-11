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
    repeated_block_count(bytes, 16) > 0
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


pub fn repeated_block_count(bytes: &[u8], block_size: usize) -> usize {
    let block_count = bytes.len() / block_size as usize;

    let mut repeated_blocks: usize = 0;
    for block_idx in range(0, block_count) {
        let block = bytes.slice(block_idx * block_size, (block_idx + 1) * block_size);
        for other_idx in range(block_idx + 1, block_count) {
            if block == bytes.slice(other_idx*block_size, (other_idx+1)*block_size) {
                repeated_blocks += 1;
            }
        }
    }

    repeated_blocks
}

#[test]
fn no_repeated_blocks() {
    let bytes = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(0, repeated_block_count(bytes.as_slice(), 3));
}

#[test]
fn one_repeated_block() {
    let bytes = vec![1, 1, 2, 3, 4, 5, 1, 1, 6, 7];
    assert_eq!(1, repeated_block_count(bytes.as_slice(), 2));
}

#[test]
fn three_repeated_blocks() {
    let bytes = vec![1, 1, 2, 3, 1, 1, 4, 5, 1, 1, 6, 7];
    assert_eq!(3, repeated_block_count(bytes.as_slice(), 2));
}
