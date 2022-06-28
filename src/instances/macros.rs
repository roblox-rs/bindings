#[macro_export]
macro_rules! impl_instance_exclusive {
    ($name:ident) => {
        use $crate::instances::{
            get_child, get_instance_property, get_string_property, set_instance_property,
            set_string_property, Instance,
        };

        #[derive(Clone)]
        pub struct $name(pub i32);

        impl Instance for $name {
            fn id(&self) -> i32 {
                self.0
            }

            fn child(&self, name: &str) -> Option<UnknownInstance> {
                match unsafe { get_child(self.0, name) } {
                    0 => None,
                    id => Some(UnknownInstance(id)),
                }
            }

            fn name(&self) -> String {
                unsafe { get_string_property(self.0, "Name") }
            }

            fn parent(&self) -> Option<UnknownInstance> {
                unsafe { get_instance_property(self.0, "Parent").map(|v| UnknownInstance(v)) }
            }

            fn set_name(&self, name: &str) {
                unsafe { set_string_property(self.0, "Name", name) }
            }

            fn set_parent<I: Instance>(&self, parent: Option<I>) {
                unsafe {
                    set_instance_property(self.0, "Parent", parent.map(|v| v.id()).unwrap_or(0))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_instance {
    ($name:ident) => {
        use $crate::instances::instance_is_a;
        use $crate::instances::UnknownInstance;

        $crate::impl_instance_exclusive!($name);

        impl std::convert::TryFrom<UnknownInstance> for $name {
            type Error = ();
            fn try_from(value: UnknownInstance) -> Result<Self, Self::Error> {
                unsafe {
                    if instance_is_a(value.id(), stringify!($name)) {
                        Ok($name(value.id()))
                    } else {
                        Err(())
                    }
                }
            }
        }

        impl From<$name> for UnknownInstance {
            fn from(value: $name) -> Self {
                UnknownInstance(value.id())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_base_part {
    ($name:ident) => {
        use $crate::datatypes::Vector3;
        use $crate::instances::{
            get_bool_property, get_float_property, get_vector3_property, set_bool_property,
            set_float_property, set_vector3_property,
        };

        impl $name {
            pub fn can_collide(&self) -> bool {
                unsafe { get_bool_property(self.id(), "CanCollide") }
            }

            pub fn set_can_collide(&self, value: bool) {
                unsafe { set_bool_property(self.id(), "CanCollide", value) }
            }

            pub fn position(&self) -> Vector3 {
                unsafe { get_vector3_property(self.id(), "Position") }
            }

            pub fn set_position(&self, vector3: Vector3) {
                unsafe { set_vector3_property(self.id(), "Position", vector3.0) }
            }

            pub fn transparency(&self) -> f64 {
                unsafe { get_float_property(self.id(), "Transparency") }
            }

            pub fn set_transparency(&self, value: f64) {
                unsafe { set_float_property(self.id(), "Transparency", value) }
            }
        }
    };
}
