extern mod xor;
use xor::Xor;

#[test]
fn test_xor_equal_length() {
    let bytes = ~[1, 2, 3];
    assert_eq!(bytes.xor([1, 1, 1]), ~[0, 3, 2]);
}

#[test]
fn test_xor_repeating_key() {
    let bytes = ~[1, 2, 3];
    assert_eq!(bytes.xor([1]), ~[0, 3, 2]);
}

#[test]
fn test_xor_single_byte() {
    let bytes = ~[1, 2, 3];
    assert_eq!(bytes.xor_byte(1u8), ~[0, 3, 2]);
}