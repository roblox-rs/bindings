use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[allow(improper_ctypes)]
extern "C" {
    fn vector3_new(x: f64, y: f64, z: f64) -> Vector3;
    fn get_vector3_x(vector: u32) -> f64;
    fn get_vector3_y(vector: u32) -> f64;
    fn get_vector3_z(vector: u32) -> f64;
}

use super::{
    add_datatype_ptr, div_datatype_ptr, div_datatype_scalar, mul_datatype_ptr, mul_datatype_scalar,
    sub_datatype_ptr,
};

#[repr(transparent)]
pub struct Vector3(pub(crate) u32);

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        unsafe { vector3_new(x, y, z) }
    }

    pub fn to_ptr(&self) -> u32 {
        self.0
    }

    pub fn x(&self) -> f64 {
        unsafe { get_vector3_x(self.to_ptr()) }
    }

    pub fn y(&self) -> f64 {
        unsafe { get_vector3_y(self.to_ptr()) }
    }

    pub fn z(&self) -> f64 {
        unsafe { get_vector3_z(self.to_ptr()) }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // floats can't currently be formatted, so convert to integer first
        f.write_str(&format!(
            "Vector3({}, {}, {})",
            self.x() as i64,
            self.y() as i64,
            self.z() as i64,
        ))
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        unsafe { add_datatype_ptr(self.0, rhs.0) }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        unsafe { sub_datatype_ptr(self.0, rhs.0) }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        unsafe { mul_datatype_ptr(self.0, rhs.0) }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        unsafe { mul_datatype_scalar(self.0, rhs) }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        unsafe { div_datatype_ptr(self.0, rhs.0) }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output {
        unsafe { div_datatype_scalar(self.0, rhs) }
    }
}
