extern crate decrypt;
extern crate serialize;

use serialize::base64::FromBase64;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new(file!().to_string() + "/../../files/set01-challenge06.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let base64 = reader.read_to_end().unwrap();
    let bytes = base64.from_base64().unwrap();
    let answer = decrypt::xor_repeating(bytes.as_slice());

    println!("key        {}", answer.key);
    println!("key text   {}", String::from_utf8_lossy(answer.key.as_slice()));
    println!("plaintext\n\n{}", String::from_utf8_lossy(answer.bytes.as_slice()));
}
