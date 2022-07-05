pub mod datatypes;
pub mod generated;
pub mod libraries;
pub mod mem;

pub use datatypes::*;
pub use generated::*;
pub use libraries::*;

#[allow(improper_ctypes)]
extern "C" {
    fn instance_new(name: &str) -> u32;

    fn clone_pointer(ptr: u32) -> u32;
    fn drop_pointer(ptr: u32);
}
