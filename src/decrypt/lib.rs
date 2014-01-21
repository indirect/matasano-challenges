extern mod extra;
extern mod xor;
extern mod english;

use std::str;
use extra::hex::{FromHex};
use xor::Xor;

pub struct Answer {
    text: ~str,
    key: ~str,
    score: int
}

pub fn xor_byte(encrypted: &str) -> Option<Answer> {
    let bytes = match encrypted.from_hex() {
        Ok(hex_bytes) => hex_bytes,
        Err(msg) => fail!(msg)
    };

    let mut possibles = range(0u8, 255u8).filter_map(|cand| {
        let xor_bytes = bytes.xor_byte(cand);
        match str::from_utf8_owned_opt(xor_bytes.clone()) {
            Some(possible) => Some(Answer {
                text: possible,
                score: english::score(xor_bytes),
                key: str::from_utf8_owned(~[cand])
            }),
            None => None
        }
    });

    possibles.max_by(|answer| answer.score )
}
