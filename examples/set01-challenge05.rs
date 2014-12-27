extern crate serialize;
extern crate xor;

use xor::Xor;
use serialize::hex::ToHex;

fn main() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let ciphered_bytes = plaintext.as_bytes().xor(key);
    println!("Plaintext:  {}", plaintext);
    println!("Ciphertext: {}", ciphered_bytes.to_hex());
}