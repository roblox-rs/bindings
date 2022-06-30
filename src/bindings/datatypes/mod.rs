mod boilerplate;
mod color3;
mod vector3;

pub use boilerplate::*;
pub use color3::*;
pub use vector3::*;

#[allow(improper_ctypes)]
extern "C" {
    fn add_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;
    fn sub_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;

    fn mul_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;
    fn div_datatype_ptr(lhs: u32, rhs: u32) -> Vector3;

    fn mul_datatype_scalar(lhs: u32, rhs: f64) -> Vector3;
    fn div_datatype_scalar(lhs: u32, rhs: f64) -> Vector3;
}
