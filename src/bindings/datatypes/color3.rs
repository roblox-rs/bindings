#[allow(improper_ctypes)]
extern "C" {
    fn color3_new(r: f64, g: f64, b: f64) -> Color3;
    fn color3_from_rgb(r: f64, g: f64, b: f64) -> Color3;
    fn color3_from_hsv(h: f64, s: f64, v: f64) -> Color3;
    fn color3_from_hex(hex: &str) -> Color3;
    fn color3_to_hex(id: u32) -> String;
    fn color3_lerp(start: u32, target: u32, alpha: f64) -> Color3;
    fn get_color3_r(vector: u32) -> f64;
    fn get_color3_g(vector: u32) -> f64;
    fn get_color3_b(vector: u32) -> f64;
}

#[repr(C)]
pub struct Color3(pub(crate) u32);

impl Color3 {
    pub fn new(r: f64, g: f64, b: f64) -> Color3 {
        unsafe { color3_new(r, g, b) }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Color3 {
        unsafe { color3_from_rgb(r, g, b) }
    }

    pub fn from_hex(hex: &str) -> Color3 {
        unsafe { color3_from_hex(hex) }
    }

    pub fn from_hsv(h: f64, s: f64, v: f64) -> Color3 {
        unsafe { color3_from_hsv(h, s, v) }
    }

    pub fn to_ptr(&self) -> u32 {
        self.0
    }

    pub fn r(&self) -> f64 {
        unsafe { get_color3_r(self.to_ptr()) }
    }

    pub fn g(&self) -> f64 {
        unsafe { get_color3_g(self.to_ptr()) }
    }

    pub fn b(&self) -> f64 {
        unsafe { get_color3_b(self.to_ptr()) }
    }

    pub fn to_hex(&self) -> String {
        unsafe { color3_to_hex(self.to_ptr()) }
    }

    pub fn lerp(&self, target: Color3, alpha: f64) -> Color3 {
        unsafe { color3_lerp(self.to_ptr(), target.to_ptr(), alpha) }
    }
}
