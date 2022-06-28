// use std::ops::Index;
mod base_part;
mod datamodel;
mod macros;
mod unknown;

pub use base_part::*;
pub use datamodel::*;
pub use unknown::*;

use crate::datatypes::Vector3;

#[allow(improper_ctypes)]
extern "C" {
    fn get_game() -> i32;

    fn get_string_property(instance: i32, name: &str) -> String;
    fn set_string_property(instance: i32, name: &str, value: &str);

    fn get_instance_property(instance: i32, name: &str) -> Option<i32>;
    fn set_instance_property(instance: i32, name: &str, parent: i32);

    fn get_bool_property(instance: i32, name: &str) -> bool;
    fn set_bool_property(instance: i32, name: &str, value: bool);

    fn get_vector3_property(instance: i32, name: &str) -> Vector3;
    fn set_vector3_property(instance: i32, name: &str, value: i32);

    fn get_float_property(instance: i32, name: &str) -> f64;
    fn set_float_property(instance: i32, name: &str, value: f64);

    fn get_child(instance: i32, name: &str) -> i32;
    fn instance_is_a(instance: i32, name: &str) -> bool;

}

pub trait Instance {
    fn id(&self) -> i32;
    fn child(&self, name: &str) -> Option<UnknownInstance>;
    fn name(&self) -> String;
    fn parent(&self) -> Option<UnknownInstance>;

    fn set_parent<I: Instance>(&self, instance: Option<I>);
    fn set_name(&self, name: &str);
}

// impl Index<String> for dyn Instance {
//     type Output = Option<UnknownInstance>;
//     fn index(&self, index: String) -> &Self::Output {
//         unsafe {
//             match get_child(self.id(), &index) {
//                 0 => &None,
//                 id => &Some(UnknownInstance(id)),
//             }
//         }
//     }
// }
