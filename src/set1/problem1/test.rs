extern mod extra;

#[test]
fn test_hex_to_bas64() {
    use extra::base64::{ToBase64, STANDARD};
    use extra::hex::{FromHex};

    let hex_source = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64 = hex_source.from_hex().unwrap().to_base64(STANDARD);
    assert_eq!(base64, ~"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

#[test]
fn test_base64_to_hex() {
    use extra::base64::{FromBase64};
    use extra::hex::{ToHex};

    let base64_source = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let hex = base64_source.from_base64().unwrap().to_hex();
    assert_eq!(hex, ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
}
