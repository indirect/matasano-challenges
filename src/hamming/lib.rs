extern mod extra;

fn to_binary(bytes: &[u8]) -> ~str {
    use std::num::ToStrRadix;

    let mut binary = ~"";

    for byte in bytes.iter() {
        binary.push_str(format!("{:0>8s}", byte.to_str_radix(2)));
    }

    binary
}

pub fn distance(one: &[u8], two: &[u8]) -> i64 {
    use extra::hex::{ToHex, FromHex};

    if (one.len() != two.len()) {
        fail!("only equal length strings")
    }

    let oneb = to_binary(one);
    let twob = to_binary(two);

    let mut dist: i64 = 0;

    for i in range(0, oneb.len()) {
        if oneb[i] != twob[i] { dist += 1 }
    }

    dist
}