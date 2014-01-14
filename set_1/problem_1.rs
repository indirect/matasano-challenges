// 1. hex to base64

extern mod extra;
use extra::base64::ToBase64;
use extra::base64::FromBase64;

fn hex_to_base64(source: &str) -> ~str {
  source.from_hex().to_base64()
}

fn base64_to_hex(source: &str) -> ~str {
  source.from_base64().to_hex()
}

trait ToHex {
  fn to_hex(&self) -> ~str;
}

impl<‘self> ToHex for &’self [u8] {
  fn to_hex(&self) -> ~str {
    let hex: @mut ~str = @mut ~””;
    for self.each |byte| {
      hex.push_str(std::u8::to_str_radix(*byte, 16));
    };
    (*hex).clone()
  }
}

trait FromHex {
  fn from_hex(&self) -> ~[u8];
}

impl<‘self> FromHex for &’self str {
  fn from_hex(&self) -> ~[u8] {
    let bytes = self.to_bytes();
    let mut nums: ~[u8] = ~[];

    for std::uint::range_step(0, bytes.len(), 2) |i| {
      let num = std::u8::parse_bytes(bytes.slice(i, i+2), 16);
      match num { Some(n) => nums.push(n), None => () }
    };

    nums
  }
}

fn main() {
  let source = ~”49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d”;
  println(fmt!(“%s”, hex_to_base64(source)));
  println(fmt!(“%s”, base64_to_hex(hex_to_base64(source))));
}

// output
// $ ./convert
// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
// 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
