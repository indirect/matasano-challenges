extern crate serialize;

use serialize::hex::FromHex;
use serialize::base64::ToBase64;

fn base64_config() -> serialize::base64::Config {
	serialize::base64::Config {
		char_set: serialize::base64::CharacterSet::Standard,
	 	newline: serialize::base64::Newline::LF,
		pad: false,
		line_length: None
	}
}

fn main() {
	let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  println!("{}", hex);

	let bytes = match hex.from_hex() {
		Ok(bytes) => bytes,
		Err(why) => panic!("String was not hexadecimal: {}", why),
	};

  let base64 = bytes.to_base64(base64_config());
	println!("{}", base64);
}
