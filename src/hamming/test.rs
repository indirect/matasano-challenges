extern mod hamming;

#[test]
fn test_hamming_distance() {
    let distance = hamming::distance("this is a test", "wokka wokka!!!");
    assert_eq!(37, distance);
}