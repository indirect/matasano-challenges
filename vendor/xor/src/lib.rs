pub trait Xor {
    fn xor(&self, other: &[u8]) -> Vec<u8>;
    fn xor_byte(&self, other: u8) -> Vec<u8>;
}

impl Xor for [u8] {
    fn xor(&self, other: &[u8]) -> Vec<u8> {
        let otherlen = other.len();
        range(0, self.len()).map(|i| {
            let oi = i % otherlen;
            self[i] ^ other[oi]
        }).collect()
    }

    fn xor_byte(&self, byte: u8) -> Vec<u8> {
        range(0, self.len()).map(|i| {
            self[i] ^ byte
        }).collect()
    }
}

#[test]
fn test_xor_equal_length() {
    let bytes = vec!(1u8, 2, 3);
    let key = vec!(1u8, 1, 1);
    let xored = bytes.xor(key.as_slice());
    assert_eq!(xored, [0, 3, 2]);
}

#[test]
fn test_xor_repeating_key() {
    let bytes = vec!(1u8, 2, 3);
    let key = vec!(1u8);
    assert_eq!(bytes.xor(key.as_slice()), [0, 3, 2]);
}

#[test]
fn test_xor_single_byte() {
    let bytes = vec!(1, 2, 3);
    assert_eq!(bytes.xor_byte(1u8), [0, 3, 2]);
}
