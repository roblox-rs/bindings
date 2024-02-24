use super::Foreign;

#[repr(C)]
pub enum RustOption<T: Foreign> {
    None,
    Some(T),
}

/// A FFI-safe string received from bindings.
/// Capacity is guaranteed to be equal to length as Luau never resizes strings.
#[repr(C)]
pub struct RustString {
    pub content: *mut u8,
    pub length: usize,
}

#[repr(C)]
pub struct RustSlice<T: Foreign> {
    pub content: *const T,
    pub length: usize,
}
