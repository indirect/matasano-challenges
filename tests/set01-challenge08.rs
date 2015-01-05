extern crate serialize;
extern crate crypto;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

#[test]
fn set01_challenge08() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge08.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());

    let ecb_lines: Vec<uint> = reader.lines().enumerate().filter_map(|tuple| {
        let (index, hex_line) = tuple;
        let bytes = hex_line.unwrap().from_hex().unwrap();
        let repeated_blocks = crypto::count_repeated_blocks(bytes.as_slice(), 16);

        if repeated_blocks > 0 {
            Some(index)
        } else {
            None
        }
    }).collect();

    // Line 132 has repeating 16 byte sections, and is therefore probably ECB
    assert_eq!(vec![132], ecb_lines);
}
