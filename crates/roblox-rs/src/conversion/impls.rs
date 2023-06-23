use std::slice;

use super::{
    foreign,
    structs::{RustOption, RustSlice, RustString},
    FromForeign, IntoForeign,
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

    fn from_foreign(value: &Self::Input) -> Self {
        match value {
            RustOption::Some(value) => Some(T::from_foreign(value)),
            RustOption::None => None,
        }
    }
}

impl FromForeign for String {
    type Input = RustString;

    fn from_foreign(input: &Self::Input) -> Self {
        unsafe { String::from_raw_parts(input.content, input.length, input.length) }
    }
}

// We cannot IntoForeign for slices (currently) as we must map the entirety of the slice
// using T::into_foreign, however that would involve creating a new slice which must be kept alive past the scope of the function.
// Not clear yet how to implement this.
// impl<T: IntoForeign> IntoForeign for &[T] {
//     type Output = RustSlice<T::Output>;

//     fn into_foreign(self) -> Self::Output {
//         RustSlice {
//             content: self.as_ptr(),
//             length: self.len(),
//         }
//     }
// }

// IntoForeign for Vec<T> is also problematic (currently) as we don't have a Foreign owned array (there is RustSlice, but it is a reference)
// impl<T: IntoForeign> IntoForeign for Vec<T> {
//     type Output = Vec<T::Output>;

//     fn into_foreign(self) -> Self::Output {
//         self.into_iter().map(|v| v.into_foreign()).collect()
//     }
// }

impl<T: FromForeign> FromForeign for Vec<T> {
    type Input = RustSlice<T::Input>;

    fn from_foreign(input: &Self::Input) -> Self {
        unsafe { slice::from_raw_parts(input.content, input.length) }
            .iter()
            .map(|v| T::from_foreign(v))
            .collect()
    }
}

// impl<T: IntoForeign> IntoForeign for &[T] {
//     type Output = RustSlice<T>;
// }

foreign!(i8 u8 i16 u16 i32 u32 f32 i64 u64 f64 RustString RustSlice<T> RustOption<T>);
