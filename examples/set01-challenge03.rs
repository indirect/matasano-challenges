extern crate decrypt;
extern crate "rustc-serialize" as serialize;

use serialize::hex::FromHex;

fn main() {
    let source = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes  = source.from_hex().unwrap();
    let answer = decrypt::xor_byte(bytes.as_slice()).unwrap();

    println!("The best score was {:?}, using the key {:?}", answer.score, answer.key);
    println!("The decoded string is: {:?}", String::from_utf8_lossy(answer.bytes.as_slice()));
}
