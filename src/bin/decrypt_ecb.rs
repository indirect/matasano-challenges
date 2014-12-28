extern crate openssl;
extern crate serialize;

use serialize::hex::{FromHex,ToHex};

fn main() {
    let args = std::os::args();

    let key = args[1].as_bytes();
    let iv = Vec::new();
    let cipher = args[2].from_hex().unwrap();
    let result = openssl::crypto::symm::decrypt(
        openssl::crypto::symm::Type::AES_128_ECB,
        key.as_slice(),
        iv,
        cipher.as_slice()
    );

    println!("ciphertext: {}", cipher.to_hex());
    println!("key:        {} ({})", key.to_hex(), String::from_utf8_lossy(key.as_slice()));
    println!("plaintext:  {}", String::from_utf8_lossy(result.as_slice()));
}
