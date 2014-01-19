pub trait Any {
    fn type_name(&self) -> &'static str;
}

impl<T: 'static> Any for T {
    fn type_name(&self) -> &'static str {
		unsafe {
			(*std::unstable::intrinsics::get_tydesc::<T>()).name
		}
    }
}
