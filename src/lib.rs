extern crate serialize;

use serialize::base64::FromBase64;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

pub fn bytes_from_data_file(filename: &str) -> Vec<u8> {
    let path = Path::new(file!().to_string() + "/../../data/" + filename);
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let base64 = reader.read_to_end().unwrap();
    base64.from_base64().unwrap()
}