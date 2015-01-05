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
        let is_ecb = crypto::ecb::detect(bytes.as_slice());

        if is_ecb {
            println!("line {} is likely encrypted using ECB", index);
        }
    }
}
