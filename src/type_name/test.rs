extern mod type_name;
use type_name::Any;

#[test]
fn test_type_name() {
	let item: ~[u8] = ~[];
	assert_eq!(item.type_name(), "~[u8]");
}