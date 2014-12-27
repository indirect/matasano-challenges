pub trait Binary for Sized? {
    fn to_binary(&self) -> String;
}

impl Binary for [u8] {
    fn to_binary(&self) -> String {
        use std::fmt;

        let binaries = Vec::from_fn(self.len(), |idx| {
            format!("{:0>8}", fmt::radix(self[idx], 2))
        });

        binaries.concat()
    }
}

#[test]
fn test_to_binary() {
    assert_eq!("10000000", [128u8].to_binary());
}

#[test]
fn test_to_binary_less_than_eight_digits() {
    assert_eq!("00100000", [32u8].to_binary());
}
