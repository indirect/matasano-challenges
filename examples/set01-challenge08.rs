extern crate hamming;
extern crate serialize;
extern crate decrypt;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

fn count_repeated_blocks(bytes: &[u8], block_size: uint) -> uint {
    let block_count = bytes.len() / block_size as uint;

    let mut repeated_blocks: uint = 0;
    for block_idx in range(0, block_count) {
        let block = bytes.slice(block_idx * block_size, (block_idx + 1) * block_size);
        for other_idx in range(block_idx + 1, block_count) {
            if block == bytes.slice(other_idx*block_size, (other_idx+1)*block_size) {
                repeated_blocks += 1;
            }
        }
    }

    repeated_blocks
}

fn main() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge08.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());

    for (index, hex_line) in reader.lines().enumerate() {
        let bytes = hex_line.unwrap().from_hex().unwrap();
        let repeated_blocks = count_repeated_blocks(bytes.as_slice(), 16);

        if repeated_blocks > 0 {
            println!("line {} repeated {} blocks", index, repeated_blocks);
        }
    }
}
