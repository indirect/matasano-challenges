trait ToUtf8 for Sized? {
    fn to_utf8(&self) -> String;
}

impl ToUtf8 for [u8] {
    fn to_utf8(&self) -> String {
       String::from_utf8(self.to_vec()).unwrap()
    }
}

#[test]
fn it_works() {
    assert_eq!("A", vec![65].to_utf8());
}
