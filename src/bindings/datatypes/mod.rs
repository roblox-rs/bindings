mod boilerplate;
mod color3;
mod rbx_script_connection;
mod vector3;

pub use boilerplate::*;
pub use color3::*;
pub use rbx_script_connection::*;
pub use vector3::*;

#[macro_export]
macro_rules! impl_datatype_drop {
    ($name:ident) => {
        $crate::impl_datatype_drop!($name, |self| {});
    };
    ($name:ident, |$self:ident| $body:block) => {
        impl Drop for $name {
            fn drop(&mut $self) {
                $body;
                unsafe { $crate::bindings::drop_pointer($self.to_ptr()) }
            }
        }
    };
}

#[allow(improper_ctypes)]
extern "C" {
    fn add_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;
    fn sub_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;

    fn mul_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;
    fn div_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;

    fn mul_datatype_scalar(lhs: u32, rhs: f64) -> Vector3;
    fn div_datatype_scalar(lhs: u32, rhs: f64) -> Vector3;
}
