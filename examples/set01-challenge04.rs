extern crate decrypt;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

fn main() {
    let path = Path::new(file!().to_string() + "/../../files/set01-challenge04.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let answers = reader.lines().filter_map(|line| decrypt::xor_byte(line.unwrap().as_slice()) );
    let best = answers.max_by(|answer| answer.score ).unwrap();

    println!("best score: {}", best.score.to_string());
    println!("decrypted: {}", best.text.trim_right());
    println!("xor key: '{}'", best.key);
}
