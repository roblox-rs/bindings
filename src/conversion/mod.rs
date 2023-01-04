mod impls;
mod structs;

/// This is a marker trait for generic bounds that statically ensures only FFI safe types can be used
pub trait Foreign {}

/// This trait is for converting to an FFI safe type.
pub trait IntoForeign {
    type Output: Foreign;

    fn into_foreign(self) -> Self::Output;
}

/// This trait is for converting from an FFI safe type.
pub trait FromForeign {
    type Input: Foreign;

    fn from_foreign(input: Self::Input) -> Self;
}

macro_rules! foreign {
	($name:ident<$($ty:ident),*> $($tt:tt)*) => {
		impl<$($ty: $crate::conversion::Foreign),*> $crate::conversion::Foreign for $name<$($ty),*> {}
		foreign!($($tt)*);
	};
	($name:ident $($tt:tt)*) => {
		impl $crate::conversion::Foreign for $name {}
		foreign!($($tt)*);
	};
	() => {};
}

use foreign;
