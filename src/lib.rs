pub mod datatypes;
pub mod generated;
pub mod libraries;
pub mod mem;

use std::panic;

pub use datatypes::*;
pub use generated::*;
pub use libraries::*;

#[allow(improper_ctypes)]
extern "C" {
    fn clone_pointer(ptr: u32) -> u32;
    fn drop_pointer(ptr: u32);
}

#[no_mangle]
unsafe fn hook() {
    panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        roblox_error(&msg);
    }));
}
