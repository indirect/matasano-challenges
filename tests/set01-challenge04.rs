extern crate decrypt;
extern crate serialize;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use serialize::hex::FromHex;

#[test]
fn set01_challenge04() {
    let path = Path::new(file!().to_string() + "/../../files/set01-challenge04.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let answers = reader.lines().filter_map(|line| {
        let hex = line.unwrap().from_hex().unwrap();
        decrypt::xor_byte(hex.as_slice())
    });
    let best = answers.max_by(|answer| answer.score ).unwrap();

    assert_eq!(best.text, "Now that the party is jumping\n");
    assert_eq!(best.key, "5");
}
