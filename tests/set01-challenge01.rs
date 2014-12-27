extern crate serialize;

use serialize::hex::FromHex;
use serialize::base64::ToBase64;

#[test]
fn set01_challenge01() {
  let hex_source = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let base64 = hex_source.from_hex().unwrap().to_base64(serialize::base64::STANDARD);
  assert_eq!(base64, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}