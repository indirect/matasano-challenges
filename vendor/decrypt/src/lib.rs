extern crate serialize;
extern crate xor;
extern crate english;
extern crate hamming;

use xor::Xor;

#[deriving(Show)]
pub struct XorByteAnswer {
    pub bytes: Vec<u8>,
    pub key: u8,
    pub score: int
}

pub fn xor_byte(bytes: &[u8]) -> Option<XorByteAnswer> {
    let possibles = range(0u8, 255).filter_map(|cand| {
        let xor_bytes = bytes.xor_byte(cand);

        match String::from_utf8(xor_bytes) {
            Ok(possible) => Some(XorByteAnswer {
                bytes: possible.as_bytes().to_vec(),
                score: english::score(possible.as_bytes()),
                key: cand
            }),
            Err(_) => None
        }
    });

    possibles.max_by(|answer| answer.score )
}

#[test]
fn test_decrypt_xor_byte() {
    use serialize::hex::FromHex;

    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cipher = encrypted.from_hex().unwrap();
    let answer = xor_byte(cipher.as_slice()).unwrap();

    assert_eq!(answer.bytes, b"Cooking MC's like a pound of bacon");
    assert_eq!(answer.key, b'X');
}

fn transpose(bytes: &[u8], num_columns: uint) -> Vec<Vec<u8>> {
    let mut transposed: Vec<Vec<u8>> = Vec::new();
    for mut idx in range(0, num_columns) {
        let mut column = Vec::new();

        while idx < bytes.len() {
            column.push(bytes[idx]);
            idx += num_columns;
        }
        transposed.push(column);
    }
    transposed
}

#[test]
fn test_transpose() {
    let testing = b"aaabbbcccdd";
    let result = transpose(testing, 3);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].len(), 4);
    assert_eq!(result[2].len(), 3);
}

#[deriving(Show)]
struct KeySize {
    value: uint,
    distance: f64
}

pub fn guess_block_size(bytes: &[u8]) -> Option<uint> {
    use std::cmp;

    let max_size = cmp::min(bytes.len() / 4, 30) + 1;

    let mut keys: Vec<KeySize> = range(1, max_size).map(|size| {
        let a = bytes.slice(0, size);
        let b = bytes.slice(size, size*2);
        let c = bytes.slice(size*2, size*3);
        let d = bytes.slice(size*3, size*4);

        let mut dist = 0;
        dist += hamming::distance(a, b);
        dist += hamming::distance(a, c);
        dist += hamming::distance(a, d);
        dist += hamming::distance(b, c);
        dist += hamming::distance(b, d);
        dist += hamming::distance(c, d);
        let norm = dist as f64 / 6f64 / size as f64;

        KeySize { value: size, distance: norm }
    }).collect();

    keys.sort_by(|a,b| a.distance.partial_cmp(&b.distance).unwrap() );

    match keys.head() {
        Some(k) => Some(k.value),
        _ => None
    }
}

#[test]
fn test_guess_block_size() {
    use serialize::hex::FromHex;

    let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = cipher.from_hex().unwrap();
    let key_size = guess_block_size(bytes.as_slice());
    assert_eq!(key_size, Some(1));
}

trait ToUtf8String for Sized? {
    fn to_utf8_string(&self) -> String;
}

impl ToUtf8String for [u8] {
    fn to_utf8_string(&self) -> String {
       String::from_utf8(self.to_vec()).unwrap()
    }
}

#[deriving(Show)]
pub struct Decrypted {
    pub key: Vec<u8>,
    pub bytes: Vec<u8>
}

pub fn xor_repeating(bytes: &[u8]) -> Option<Decrypted> {
    let key_size = match guess_block_size(bytes) {
        Some(size) => size,
        None => return None
    };

    let mut keys: Vec<u8> = Vec::new();
    let mut cols: Vec<Vec<u8>> = Vec::new();

    for col in transpose(bytes, key_size).iter() {
        match xor_byte(col.as_slice()) {
            Some(answer) => {
                keys.push(answer.key);
                cols.push(answer.bytes);
            },
            None => return None
        }
    }

    let mut text = Vec::new();
    let col_len = cols[0].len();
    for i in range(0, col_len) {
        for col in cols.iter() {
            match col.get(i) {
                Some(byte) => text.push(byte.clone()),
                None => {}
            }
        }
    }

    Some(Decrypted{bytes: text, key: keys})
}

#[test]
fn test_decrypt_xor_repeating_single() {
    use serialize::hex::FromHex;
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let hex = ciphertext.from_hex().unwrap();
    let answer = xor_repeating(hex.as_slice());
    assert_eq!(answer.key, b"X");
    assert_eq!(answer.bytes, b"Cooking MC's like a pound of bacon");
}

#[test]
fn test_decrypt_xor_repeating_triple() {
    use serialize::hex::FromHex;
    let ciphertext = "1d2b2c3a632c3a632469372d202d2269372d2837652030652c2d263b3a353d262169342c3d2b652863372c332028372c27246522263c676306282d65302c3069252c2e36372c632c3d632a3c377a";
    let hex = ciphertext.from_hex().unwrap();
    let answer = xor_repeating(hex.as_slice());
    assert_eq!(answer.key, b"ICE");
    assert_eq!(answer.bytes, b"This is a thing that is encrypted with a repeating key. Can you figure it out?");
}
