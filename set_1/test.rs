use std::rand::{task_rng, Rng};
use std::io::buffered::BufferedReader;
use std::io;


fn main() {
  println!("Hi. What's your name?");

  let mut reader = BufferedReader::new(io::stdin());
  println!("{}", type_of(&reader));
  let line = reader.read_line();
  println!("{}", type_of(&line));
    println!("{}", line.unwrap());

    let mut rng = task_rng();
    let n = rng.gen_range(0, 10);
    println!("{}", type_of(&n));

// match line {
//   Some(~"\n")  => println!("\nwat?"),
//   Some(thing)  => println!("You said '{}'", thing.trim_right_chars(&'\n')),
//   None         => println!("How'd you do that")
// }
}

#[test]
fn test_hex_to_bas64() {
    let hex_source = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    assert_eq!(hex_source.hex_to_bytes().bytes_to_base64(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
}

#[test]
fn test_base64_to_hex() {
    let hex_source = ~"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    assert_eq!(hex_source.hex_to_bytes().bytes_to_base64(),
    let base64_source = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(base64_source.base64_to_bytes().to_hex())
}

fn type_of<T>(_: &T) -> &'static str {
   unsafe {
       (*std::unstable::intrinsics::get_tydesc::<T>()).name
   }
}
