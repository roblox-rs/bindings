// Unimplemented datatypes.
macro_rules! unimplemented_datatype {
    ($name:ident) => {
        #[repr(transparent)]
        pub struct $name(pub(crate) u32);

        impl $name {
            pub fn to_ptr(&self) -> u32 {
                self.0
            }
        }

        crate::impl_datatype_drop!($name);
    };
}

unimplemented_datatype!(Function);
unimplemented_datatype!(CatalogSearchParams);
unimplemented_datatype!(RaycastParams);
unimplemented_datatype!(DockWidgetPluginGuiInfo);
unimplemented_datatype!(OverlapParams);
unimplemented_datatype!(Vector3int16);
unimplemented_datatype!(Region3);
unimplemented_datatype!(Vector2);
unimplemented_datatype!(Ray);
unimplemented_datatype!(RbxScriptSignal);
// unimplemented_datatype!(RbxScriptConnection);
unimplemented_datatype!(Objects);
unimplemented_datatype!(Rect);
unimplemented_datatype!(Axes);
unimplemented_datatype!(UDim2);
unimplemented_datatype!(Faces);
unimplemented_datatype!(CFrame);
unimplemented_datatype!(RaycastResult);
unimplemented_datatype!(ProtectedString);
unimplemented_datatype!(RotationCurveKey);
unimplemented_datatype!(NumberRange);
unimplemented_datatype!(Region3int16);
unimplemented_datatype!(PhysicalProperties);
unimplemented_datatype!(BinaryString);
unimplemented_datatype!(Content);
unimplemented_datatype!(BrickColor);
unimplemented_datatype!(ColorSequence);
unimplemented_datatype!(NumberSequence);
unimplemented_datatype!(FloatCurveKey);
unimplemented_datatype!(Font);
unimplemented_datatype!(QDir);
unimplemented_datatype!(QFont);
unimplemented_datatype!(DateTime);
unimplemented_datatype!(TweenInfo);
unimplemented_datatype!(UDim);
