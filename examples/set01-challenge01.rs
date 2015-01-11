extern crate "rustc-serialize" as serialize;

use serialize::hex::FromHex;
use serialize::base64::ToBase64;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("Hex:    {:?}", hex);

    let bytes = match hex.from_hex() {
        Ok(bytes) => bytes,
        Err(why) => panic!("String was not hexadecimal: {:?}", why),
    };

    println!("Text:   {:?}", String::from_utf8(bytes.clone()).unwrap());

    let base64 = bytes.to_base64(serialize::base64::STANDARD);
    println!("Base64: {:?}", base64);
}
