extern mod extra;
extern mod hex;
use hex::{from_hex, to_hex};

#[test]
fn test_from_hex() {
    let hex_bytes = from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let ascii = std::str::from_utf8_owned(hex_bytes);
    assert_eq!(~"I'm killing your brain like a poisonous mushroom", ascii);
}

#[test]
fn test_to_hex() {
    let hex_bytes = to_hex(bytes!("I'm killing your brain like a poisonous mushroom"));
    assert_eq!(~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", hex_bytes);
}

#[test]
fn test_hex_to_base64() {
    use extra::base64::{ToBase64, STANDARD};

    let hex_bytes = from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64 = hex_bytes.to_base64(STANDARD);
    assert_eq!(~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64);
}
