#![allow(unstable)]
extern crate crypto;

#[test]
fn set02_challenge11() {
    let plain: [u8; 256] = [50; 256];
    let mut seen: Vec<crypto::Cipher> = Vec::new();

    while (seen.len() < 2) {
        let bytes = crypto::random_oracle(&plain[]);
        let cipher = crypto::detect_cipher(&bytes[]);
        if !seen.contains(&cipher) {
            seen.push(cipher);
        }
    }
}
