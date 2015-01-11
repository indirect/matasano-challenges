extern crate decrypt;
extern crate "rustc-serialize" as serialize;

use serialize::base64::FromBase64;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

#[test]
fn set01_challenge06() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge06.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let base64 = reader.read_to_end().unwrap();
    let bytes = base64.from_base64().unwrap();
    let answer = decrypt::xor_repeating(bytes.as_slice()).unwrap();
    let key = String::from_utf8_lossy(answer.key.as_slice());
    let plaintext = String::from_utf8_lossy(answer.bytes.as_slice());

    assert_eq!("Terminator X: Bring the noise", key);
    assert!(plaintext.contains("Spaghetti with a spoon! Come on and say it!"));
}
