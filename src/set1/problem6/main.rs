// #[feature(phase)];
//
// #[phase(syntax)]
// extern mod p;

extern mod decrypt;
extern mod extra;
extern mod hamming;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use extra::base64::{FromBase64};
// use p;

fn fcmp(a: f64, b: f64) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

struct Key {
    value: uint,
    dist: f64
}

fn main() {
    let path = Path::new(file!() + "/../gist.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let encoded = reader.read_to_str();
    let encoded_bytes = encoded.from_base64().unwrap().to_str();

    let mut keys: ~[Key] = std::vec::build(None, |push| {
        for cand in range(2u, 41u) {
            let a = encoded_bytes.slice(0, cand);
            let b = encoded_bytes.slice(cand, cand*2);
            let c = encoded_bytes.slice(cand*2, cand*3);
            let d = encoded_bytes.slice(cand*3, cand*4);

            let mut dist = 0;
            dist += hamming::distance(a, b);
            dist += hamming::distance(a, c);
            dist += hamming::distance(a, d);
            dist += hamming::distance(b, c);
            dist += hamming::distance(b, d);
            dist += hamming::distance(c, d);
            let norm = dist as f64 / cand as f64 / 6f64;

            push(Key { value: cand, dist: norm });
        };
    });
    keys.sort_by(|a, b| fcmp(b.dist, a.dist) );

    let likely_keys = keys.slice_from(keys.len() - 5);
    for key in likely_keys.iter() {
        println!("length {0:2u} distance {1:s}", key.value, std::f64::to_str_exact(key.dist, 3));
    };
}
