extern mod extra;
extern mod xor;

use extra::hex::{ToHex};
use xor::Xor;

fn main() {
    let args = std::os::args();
    let plain  = args.get(1).unwrap().as_bytes();
    let key    = args.get(2).unwrap().as_bytes();
    let cipher = plain.xor(key);

    println!("plaintext:  {}", plain.to_hex());
    println!("key:        {}", key.to_hex());
    println!("ciphertext: {}", cipher.to_hex());
}
