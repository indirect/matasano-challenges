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
    use std::collections::HashMap;
    use std::iter::repeat;

    let key = crypto::random_key();
    let input = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let mut block_size: usize = 0;

    // Block size detection
    let mut prev_first: u8 = 0;
    for size in range(1, input.len()) {
        let first_byte = unknown_oracle(&key[], &input[0..size])[0];

        if prev_first == first_byte {
            block_size = size - 1;
            println!("Block size is likely {} bytes", block_size);
            break;
        }

        prev_first = first_byte;
    }

    // ECB detection
    let ciphertext = unknown_oracle(&key[], &input[]);
    if crypto::ecb::detect(&ciphertext[]) {
        println!("Cipher used is ECB");
    } else {
        println!("Cipher used is not ECB?!?!");
    }

    // Discover first byte
    let base: Vec<u8> = repeat(50).take(block_size - 1).collect();
    let mut blocks: HashMap<Vec<u8>, u8> = HashMap::new();
    for byte in range(0u8, 255) {
        let mut input = base.clone();
        input.push(byte);
        let block = unknown_oracle(&key[], &input[])[0..16].to_vec();
        blocks.insert(block, byte);
    }

    let cipher_block = unknown_oracle(&key[], &base[])[0..16].to_vec();
    println!("First byte is {}", blocks.get(&cipher_block).unwrap());
}
