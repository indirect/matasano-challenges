extern mod binary;

use binary::Binary;

#[test]
fn test_to_binary() {
    assert_eq!(~"10000000", [128u8].to_binary());
}

#[test]
fn test_to_binary_less_than_eight_digits() {
    assert_eq!(~"00100000", [32u8].to_binary());
}