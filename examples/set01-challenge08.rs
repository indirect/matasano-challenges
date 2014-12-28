extern crate serialize;
extern crate crypto;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge08.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());

    for (index, hex_line) in reader.lines().enumerate() {
        let bytes = hex_line.unwrap().from_hex().unwrap();
        let repeated_blocks = crypto::count_repeated_blocks(bytes.as_slice(), 16);

        if repeated_blocks > 0 {
            println!("line {} repeated {} blocks", index, repeated_blocks);
        }
    }
}
