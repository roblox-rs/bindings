pub const RUST_VEC: &str = "\
#[repr(C)]
pub struct RustVec<T> {
    content: *mut T,
    length: usize,
}

impl<T> From<Vec<T>> for RustVec<T> {
    fn from(mut vec: Vec<T>) -> RustVec<T> {
		let content = vec.as_mut_ptr();
		let length = vec.len();
		let capacity = vec.capacity();
		std::mem::forget(vec);
        assert!(length == capacity);
        RustVec { content, length }
    }
}

impl<T> From<RustVec<T>> for Vec<T> {
    fn from(string: RustVec<T>) -> Vec<T> {
        unsafe { Vec::from_raw_parts(string.content, string.length, string.length) }
    }
}
";

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
/// A FFI-safe string slice to pass to bindings.
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
";

pub const RUST_STRING: &str = "\
/// A FFI-safe string received from bindings.
/// Capacity is guaranteed to be equal to length as Luau never resizes strings.
#[repr(C)]
pub struct RustString {
	content: *mut u8,
	length: usize,
}

impl From<RustString> for String {
	fn from(string: RustString) -> String {
		unsafe {
			String::from_raw_parts(string.content, string.length, string.length)
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

pub const RUST_INSTANCE_CUSTOM_IMPL: &str = "\
#[repr(transparent)]
pub struct $name(u32);

impl Clone for $name {
	fn clone(&self) -> Self {
		unsafe { Self(clone_pointer(self.to_ptr())) }
	}
}

impl Drop for $name {
	fn drop(&mut self) {
		unsafe { drop_pointer(self.to_ptr()) }
	}
}

impl From<$name> for LuaValue {
	fn from(value: $name) -> LuaValue {
		unsafe { std::mem::transmute::<_, LuaValue>(value) }
	}
}
";

pub const RUST_ROBLOX_ENUM_MACRO: &str = "\
macro_rules! roblox_macro {
    ($name:ident; { $($field:ident = $value:expr),*, }) => {
		#[allow(non_camel_case_types)]
		#[repr(C)]
        pub enum $name {
            $(
                $field = $value
            ),*
        }
    }
}";
