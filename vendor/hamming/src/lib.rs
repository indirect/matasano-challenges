pub fn distance(one: &[u8], two: &[u8]) -> i64 {
    if one.len() != two.len() {
        panic!("Hamming distance requires equal length strings")
    }

    let mut dist = 0i64;

    for i in range(0, one.len()) {
        let mut byte_diff = one[i] ^ two[i];
        while byte_diff > 0 {
            dist += 1;
            byte_diff = byte_diff & byte_diff - 1;
        }
    }

    dist
}

#[test]
fn test_hamming_distance() {
    let distance = distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    assert_eq!(37, distance);
}
