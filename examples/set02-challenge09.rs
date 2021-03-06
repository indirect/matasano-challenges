extern crate crypto;

use crypto::pkcs7;

fn main() {
    println!("Going to pad string YELLOW SUBMARINE with a block size of 20.");

    let submarine = "YELLOW SUBMARINE".as_bytes();
    println!("Bytes  ({:?}): {:?}", submarine.len(), submarine);

    let padded_submarine = pkcs7::pad(submarine.as_slice(), 20);
    println!("Padded ({:?}): {:?}", padded_submarine.len(), padded_submarine);
}