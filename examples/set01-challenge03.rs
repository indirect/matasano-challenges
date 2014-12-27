extern crate decrypt;

fn main() {
    let source = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let answer = decrypt::xor_byte(source).unwrap();

    println!("The best score was {}, using the key {}", answer.score, answer.key);
    println!("The decoded string is: {}", answer.text);
}
