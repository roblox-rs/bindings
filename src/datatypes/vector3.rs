use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[allow(improper_ctypes)]
extern "C" {
    fn add_vector3(lhs: i32, rhs: i32) -> Vector3;
    fn sub_vector3(lhs: i32, rhs: i32) -> Vector3;

    fn mul_vector3(lhs: i32, rhs: i32) -> Vector3;
    fn div_vector3(lhs: i32, rhs: i32) -> Vector3;

    fn mul_vector3_f(lhs: i32, rhs: f64) -> Vector3;
    fn div_vector3_f(lhs: i32, rhs: f64) -> Vector3;

    fn get_vector3(x: f64, y: f64, z: f64) -> Vector3;
    fn get_vector3_f(vec: i32, e: &str) -> f64;
}

#[repr(transparent)]
pub struct Vector3(pub i32);

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            // floats can't currently be formatted, so convert to integer first
            f.write_str(&format!(
                "Vector3({}, {}, {})",
                get_vector3_f(self.0, "x") as i64,
                get_vector3_f(self.0, "y") as i64,
                get_vector3_f(self.0, "z") as i64,
            ))
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        unsafe { get_vector3(x, y, z) }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        unsafe { add_vector3(self.0, rhs.0) }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        unsafe { sub_vector3(self.0, rhs.0) }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        unsafe { mul_vector3(self.0, rhs.0) }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        unsafe { mul_vector3_f(self.0, rhs) }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        unsafe { div_vector3(self.0, rhs.0) }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output {
        unsafe { div_vector3_f(self.0, rhs) }
    }
}
