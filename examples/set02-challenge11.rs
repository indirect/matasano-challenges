#![allow(unstable)]
extern crate crypto;

fn main() {
    let plain: [u8; 256] = [50; 256];
    for i in (0us..20) {
        let bytes = crypto::random_oracle(&plain[]);
        let cipher = crypto::detect_cipher(&bytes[]);
        println!("line {} was {:?}", i+1, cipher);
    }
}
