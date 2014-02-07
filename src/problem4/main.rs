extern mod decrypt;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

fn main() {
    let path = Path::new(file!() + "/../gist.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let mut answers = reader.lines().filter_map(|line| decrypt::xor_byte(line) );
    let best = answers.max_by(|answer| answer.score ).unwrap();

    println!("best score: {}", best.score.to_str());
    println!("decrypted: {}", best.text.trim_right());
    println!("xor key: '{}'", best.key);
}
