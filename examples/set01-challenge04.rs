extern crate decrypt;
extern crate "rustc-serialize" as serialize;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use serialize::hex::FromHex;

fn main() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge04.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let answers = reader.lines().filter_map(|line| {
        let hex = line.unwrap().from_hex().unwrap();
        decrypt::xor_byte(hex.as_slice())
    });
    let best = answers.max_by(|answer| answer.score ).unwrap();

    println!("best score: {:?}", best.score.to_string());
    println!("decrypted: {:?}", best.bytes);
    println!("xor key: '{:?}'", best.key);
}
