pub const RUST_OPTION: &str = "\
#[repr(C)]
pub enum RustOption<T> {
	None,
	Some(T),
}

impl<T> From<Option<T>> for RustOption<T> {
	fn from(option: Option<T>) -> RustOption<T> {
		match option {
			Some(value) => RustOption::Some(value),
			None => RustOption::None,
		}
	}
}

impl<T> From<RustOption<T>> for Option<T> {
	fn from(option: RustOption<T>) -> Option<T> {
		match option {
			RustOption::Some(value) => Some(value),
			RustOption::None => None,
		}
	}
}
";

pub const RUST_STR: &str = "\
#[repr(C)]
pub struct RustStr {
	content: *const u8,
	length: usize,
}

impl From<&str> for RustStr {
	fn from(string: &str) -> RustStr {
		RustStr {
			content: string.as_ptr(),
			length: string.len(),
		}
	}
}

impl From<RustStr> for String {
	fn from(string: RustStr) -> String {
		unsafe {
			std::str::from_utf8(std::slice::from_raw_parts(string.content, string.length)).unwrap().to_string()
		}
	}
}
";

pub const ROBLOX_CREATABLE: &str = "\
pub trait RobloxCreatable {
	fn new() -> Self;
}

macro_rules! creatable {
	($($name:ident)*) => {
		$(
			impl RobloxCreatable for $name {
				fn new() -> $name {
					unsafe {{ Self(instance_new(stringify!($name))) }}
				}
			}
		)*
	}
}
";

pub const EXCLUSIVE_INSTANCE: &str = "\
macro_rules! impl_instance_exclusive {
	($name:ident) => {
		impl_instance!($name);

		impl std::convert::TryFrom<Instance> for $name {
			type Error = ();
			fn try_from(value: Instance) -> Result<Self, Self::Error> {
				if value.is_a(stringify!($name)) {
					unsafe { Ok(std::mem::transmute::<_, $name>(value)) }
				} else {
					Err(())
				}
			}
		}
	}
}
";
