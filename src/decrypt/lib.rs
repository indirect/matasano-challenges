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
    let mut best: Answer = Answer { score: 0, text: ~"", key: ~"" };

    for cand in range(0u8, 255u8) {
        let attempt = encrypted.from_hex().unwrap().xor_byte(cand);
        let score = english::score(attempt);
        let atstr = str::from_utf8_owned_opt(attempt);

        if atstr.is_some() {
            if score > best.score {
                best.score = score;
                best.text = atstr.unwrap().clone();
                best.key = str::from_utf8_owned(~[cand]);
            }
        }
    }

    if best.score > 0 {
        Some(best)
    } else {
        None
    }
}
