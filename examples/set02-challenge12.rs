#![allow(unstable)]
extern crate crypto;
extern crate matasano_challenges;
extern crate "rustc-serialize" as serialize;

fn main() {
    let data = matasano_challenges::bytes_from_data_file("set02-challenge12.txt");
    let mut plain = b"my string".to_vec();
    plain.push_all(&data[]);
    let input = crypto::pkcs7::pad(&plain[], 16);
    let key = crypto::random_key();

    let ciphertext = crypto::ecb::encrypt(&key[], &input[]);
    println!("{:?}", ciphertext);
}