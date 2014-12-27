pub trait Any {
    fn type_name(&self) -> &'static str;
}

impl<T: 'static> Any for T {
    fn type_name(&self) -> &'static str {
        unsafe {
            (*std::intrinsics::get_tydesc::<T>()).name
        }
    }
}

#[test]
fn test_type_name() {
    let item: Vec<u8> = vec![];
    assert_eq!(item.type_name(), "collections::vec::Vec<u8>");
}