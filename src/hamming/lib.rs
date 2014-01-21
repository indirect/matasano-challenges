pub fn distance(one: &[u8], two: &[u8]) -> i64 {
    if (one.len() != two.len()) {
        fail!("only equal length strings")
    }

    let mut dist: i64 = 0;

    for i in range(0, one.len()) {
        let mut byte_diff = one[i] ^ two[i];
        while (byte_diff > 0) {
            dist += 1;
            byte_diff = byte_diff & byte_diff - 1;
        }
    }

    dist
}