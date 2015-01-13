#![allow(unstable)]
extern crate crypto;
extern crate matasano_challenges;
extern crate "rustc-serialize" as serialize;

fn unknown_oracle(key: &[u8], input: &[u8]) -> Vec<u8> {
    let mut plain_bytes = input.to_vec();
    let unknown_bytes = matasano_challenges::bytes_from_data_file("set02-challenge12.txt");
    plain_bytes.push_all(&unknown_bytes[]);

    let padded_bytes = crypto::pkcs7::pad(&plain_bytes[], 16);
    crypto::ecb::encrypt(&key[], &padded_bytes[])
}

fn main() {
    let key = crypto::random_key();
    let input = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    // Block size detection
    let mut first_byte = 0;

    for size in range(1, input.len()) {
        let ciphertext = unknown_oracle(&key[], &input[0..size]);

        if first_byte == ciphertext[0] {
            println!("1. Block size is likely {} bytes", size - 1);
            break;
        }

        first_byte = ciphertext[0];
    }
}
