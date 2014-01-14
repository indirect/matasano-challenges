extern mod extra;
use extra::base64::{ToBase64, FromBase64, STANDARD};
use std::str;

fn main() {
    let bsf_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let decoded = match bsf_str.from_base64() {
        Ok(bytes) => {
            for x in bytes.iter() {
                println!("{}", *x);
            }
            str::from_utf8_owned_opt(bytes)
        },
        Err(msg) => { println!("decode base64 failed: {}", msg); return }
    };
    match decoded {
        Some(msg) => println!("decoded from base64: {}", msg),
        None => println!("nothing decoded :(")
    }
}

