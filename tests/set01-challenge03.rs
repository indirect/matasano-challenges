extern crate decrypt;

#[test]
fn set01_challenge03() {
    let source = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let answer = decrypt::xor_byte(source).unwrap();

    assert_eq!("X", answer.key);
    assert_eq!("Cooking MC's like a pound of bacon", answer.text);
}
