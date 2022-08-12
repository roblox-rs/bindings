#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "print"]
    pub fn roblox_print(text: &str);
    #[link_name = "error"]
    pub fn roblox_error(text: &str) -> !;
}

#[macro_export]
macro_rules! println {
    ($($tts:tt)*) => {
        unsafe {{ roblox_print(&format!($($tts)*)) }};
    }
}
