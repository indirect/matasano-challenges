pub fn distance(one_str: &str, two_str: &str) -> i64 {
    let one = one_str.as_bytes();
    let two = two_str.as_bytes();

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
    let distance = distance("this is a test", "wokka wokka!!!");
    assert_eq!(37, distance);
}
