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
    fn drop_pointer(ptr: u32) -> u32;

    fn get_string_property(instance: u32, name: &str) -> String;
    fn set_string_property(instance: u32, name: &str, value: &str);

    fn get_instance_property(instance: u32, name: &str) -> Option<u32>;
    fn set_instance_property(instance: u32, name: &str, parent: u32);

    fn get_bool_property(instance: u32, name: &str) -> bool;
    fn set_bool_property(instance: u32, name: &str, value: bool);

    fn get_datatype_property(instance: u32, name: &str) -> u32;
    fn set_datatype_property(instance: u32, name: &str, value: u32);

    fn get_float_property(instance: u32, name: &str) -> f64;
    fn set_float_property(instance: u32, name: &str, value: f64);

    fn get_child(instance: u32, name: &str) -> u32;
    fn instance_is_a(instance: u32, name: &str) -> bool;
}
