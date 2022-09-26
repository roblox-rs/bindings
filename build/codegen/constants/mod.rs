pub mod luau;
pub mod rust;

pub const MULTI_VALUE_SUPPORT: bool = cfg!(feature = "multivalue");
