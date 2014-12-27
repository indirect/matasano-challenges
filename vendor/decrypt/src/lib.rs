extern crate serialize;
extern crate xor;
extern crate english;

use serialize::hex::FromHex;
use xor::Xor;

pub struct Answer {
    pub text: String,
    pub key: String,
    pub score: int
}

pub fn xor_byte(bytes: &[u8]) -> Option<Answer> {
    let possibles = range(0u8, 255).filter_map(|cand| {
        let xor_bytes = bytes.xor_byte(cand);

        match String::from_utf8(xor_bytes.clone()) {
            Ok(possible) => Some(Answer {
                text: possible,
                score: english::score(&xor_bytes),
                key: String::from_utf8(vec!(cand)).unwrap()
            }),
            Err(_) => None
        }
    });

    possibles.max_by(|answer| answer.score )
}


#[test]
fn test_decrypt_xor_byte() {
    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cipher = encrypted.from_hex().unwrap();
    let answer = xor_byte(cipher.as_slice()).unwrap();

    assert_eq!(answer.text, "Cooking MC's like a pound of bacon");
    assert_eq!(answer.key, "X");
}
