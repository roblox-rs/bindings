use super::{
    foreign,
    structs::{RustOption, RustSlice, RustString},
    Foreign, FromForeign, IntoForeign,
};

impl<T: IntoForeign> IntoForeign for Option<T> {
    type Output = RustOption<T::Output>;

    fn into_foreign(self) -> Self::Output {
        match self {
            Some(value) => RustOption::Some(value.into_foreign()),
            None => RustOption::None,
        }
    }
}

impl<T: FromForeign> FromForeign for Option<T> {
    type Input = RustOption<T::Input>;

    fn from_foreign(value: Self::Input) -> Self {
        match value {
            RustOption::Some(value) => Some(T::from_foreign(value)),
            RustOption::None => None,
        }
    }
}

impl FromForeign for String {
    type Input = RustString;

    fn from_foreign(input: Self::Input) -> Self {
        unsafe { String::from_raw_parts(input.content, input.length, input.length) }
    }
}

impl<T: Foreign> IntoForeign for &[T] {
    type Output = RustSlice<T>;

    fn into_foreign(self) -> Self::Output {
        RustSlice {
            content: self.as_ptr(),
            length: self.len(),
        }
    }
}

impl<T: Foreign> FromForeign for Vec<T> {
    type Input = RustSlice<T>;

    fn from_foreign(input: Self::Input) -> Self {}
}

// impl<T: IntoForeign> IntoForeign for &[T] {
//     type Output = RustSlice<T>;
// }

foreign!(i8 u8 i16 u16 i32 u32 f32 i64 u64 f64 RustString RustSlice<T> RustOption<T>);
