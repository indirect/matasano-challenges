extern mod hamming;

#[test]
fn test_hamming_distance() {
    let distance = hamming::distance(bytes!("this is a test"), bytes!("wokka wokka!!!"));
    assert_eq!(37, distance);
}