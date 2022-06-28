crate::impl_instance_exclusive!(UnknownInstance);
// pub struct UnknownInstance(pub i32);

// impl Instance for UnknownInstance {
//     fn id(&self) -> i32 {
//         self.0
//     }

//     fn child(&self, name: &str) -> Option<UnknownInstance> {
//         match unsafe { get_child(self.0, name) } {
//             0 => None,
//             id => Some(UnknownInstance(id)),
//         }
//     }

//     fn name(&self) -> String {
//         unsafe { get_string_property(self.0, "Name") }
//     }

//     fn parent(&self) -> Option<UnknownInstance> {
//         unsafe {
//             match get_parent(self.0) {
//                 0 => None,
//                 id => Some(UnknownInstance(id)),
//             }
//         }
//     }
// }
