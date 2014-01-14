extern mod extra;
use extra::hex::{FromHex};

pub trait Xor {
    fn xor(&self, other: &[u8]) -> ~[u8];
}

impl<'a> Xor for &'a [u8] {
    fn xor(&self, other: &[u8]) -> ~[u8] {
        self.iter().zip(other.iter())
            .map(|(a, b)| { a.bitxor(b) })
            .collect()
    }
}

fn score(bytes: &[u8]) -> int {
    bytes.iter().fold(0, |s, &x| {
        match x {
            // letters are just a frequency table from the internet
            69 | 101 => s + 120,
            84 | 116 => s + 90,
            65 | 97 => s + 80,
            73 | 105 => s + 80,
            78 | 110 => s + 80,
            79 | 111 => s + 80,
            83 | 115 => s + 80,
            72 | 104 => s + 64,
            82 | 114 => s + 62,
            68 | 100 => s + 44,
            76 | 108 => s + 40,
            85 | 117 => s + 34,
            67 | 99 => s + 30,
            77 | 109 => s + 30,
            70 | 102 => s + 25,
            87 | 119 => s + 20,
            89 | 121 => s + 20,
            71 | 103 => s + 17,
            80 | 112 => s + 17,
            66 | 98 => s + 16,
            86 | 118 => s + 12,
            75 | 107 => s + 8,
            81 | 113 => s + 5,
            74 | 106 => s + 4,
            88 | 120 => s + 4,
            90 | 122 => s + 2,
            // first we count \t\n\r as tiny but okay
            9 | 10 | 13 => s + 1,
            // spaces are pretty likely
            32 => s + 50,
            // other ASCII characters are okay I guess
            33..126 => s + 1,
            // then we count any control chars as death
            0..31 => s - 10000,
            // anything we don't know about yet is probably bad
            _ => s - 10000
        }
    })
}

fn main() {
    use std::str;
    let encrypted = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut best_score = 0;
    let mut best = ~"";
    let mut key = ~"";

    for i in range(0, 255) {
        let mut cand: ~[u8] = ~[];
        for _ in range(0, encrypted.len()) { cand.push(i as u8) };
        let attempt = encrypted.from_hex().unwrap().xor(cand);
        let score = score(attempt);
        let atstr = str::from_utf8_owned_opt(attempt);

        if atstr.is_some() {
            if score >= best_score {
                best_score = score;
                best = atstr.unwrap().clone();
                key = str::from_utf8_owned(cand.slice_to(1).to_owned());
            }
        }
    }

    println!("best score: {}", best_score);
    println!("decrypted: {}", best)
    println!("xor key: {}", key)
}

#[test]
fn test_score() {
    use std::str;
    let encrypted = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut best_score = 0;
    let mut best = ~"";
    let mut key = ~"";

    for i in range(0, 255) {
        let mut cand: ~[u8] = ~[];
        for _ in range(0, encrypted.len()) { cand.push(i as u8) };
        let attempt = encrypted.from_hex().unwrap().xor(cand);
        let score = score(attempt);
        let atstr = str::from_utf8_owned_opt(attempt);

        if atstr.is_some() {
            if score >= best_score {
                best_score = score;
                best = atstr.unwrap().clone();
                key = str::from_utf8_owned(cand.slice_to(1).to_owned());
            }
        }
    }
    assert_eq!(best, ~"Cooking MC's like a pound of bacon")
    assert_eq!(key, ~"X")
}

// rustc --test problem_3.rs && ./problem_3