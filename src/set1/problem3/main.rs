extern mod decrypt;

fn main() {
    let encrypted = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    match decrypt::xor_byte(encrypted) {
        Some(answer) => {
            println!("best score: {}", answer.score);
            println!("decrypted: {}", answer.text)
            println!("xor key: {}", answer.key)
        },
        None => println!("no valid utf8 options found, sorry")
    }
}
