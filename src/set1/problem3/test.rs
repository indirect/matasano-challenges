extern mod decrypt;

#[test]
fn test_problem3_answer() {
    let encrypted = ~"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let answer = decrypt::xor_byte(encrypted).unwrap();

    assert_eq!(answer.text, ~"Cooking MC's like a pound of bacon")
    assert_eq!(answer.key, ~"X")
}
