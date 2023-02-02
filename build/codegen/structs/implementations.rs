use std::ops::Deref;

use crate::codegen::stream::{note, Stream};

use super::{MemberImplementation, RenderData};

pub struct Implementation(Box<dyn MemberImplementation>);

impl<T: MemberImplementation + 'static> From<T> for Implementation {
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl Deref for Implementation {
    type Target = Box<dyn MemberImplementation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct StaticFunction(pub Option<&'static str>, pub Option<&'static str>);

create_impl!(StaticFunction; |self, { arguments, namespace_name, member_name, .. }| {
    let namespace_name = self.0.unwrap_or(namespace_name);
    let member_name = self.1.unwrap_or(member_name);
    let arguments = arguments.join(", ");

    format!("{namespace_name}.{member_name}({arguments})")
});

pub struct StaticProperty(pub Option<&'static str>, pub Option<&'static str>);

create_impl!(StaticProperty; |self, { namespace_name, member_name, .. }| {
    let namespace_name = self.0.unwrap_or(namespace_name);
    let member_name = self.1.unwrap_or(member_name);

    format!("{namespace_name}.{member_name}")
});

pub struct PropertyGetter;

create_impl!(PropertyGetter; |self, { arguments, member_name, .. }| {
    let this = &arguments[0];
    format!("{this}.{member_name}")
});

pub struct PropertySetter;

create_impl!(PropertySetter; |self, { arguments, member_name, .. }| {
    let this = &arguments[0];
    let value = &arguments[1];
    format!("{this}.{member_name} = {value};")
});

pub struct Callback;

create_impl!(Callback; |self, { arguments, member_name, .. }| {
    let this = &arguments[0];
    let value = &arguments[1];
    format!("{this}.{member_name} = {value};")
});

pub struct Method;

create_impl!(Method; |self, { arguments, member_name, .. }| {
    let this = &arguments[0];
    let arguments = arguments[1..].join(", ");
    format!("{this}:{member_name}({arguments})")
});

pub struct Event;

create_impl!(Event; |self, { arguments, member_name, parameters, .. }| {
    let this = &arguments[0];
    let function = &arguments[1];
    let data = &parameters[1][0];
    let vtable = &parameters[1][1];
    format!("createConnection({data}, {vtable}, {this}.{member_name}:Connect({function}))")
});

pub struct PointerConversion;

create_impl!(PointerConversion; |self, { arguments, .. }| {
    // This allows converting rust types into LuaValue
    // This is effectively a no-op as the bindings will convert values for us.
    arguments[0].clone()
});

pub struct PrimitiveConversion(pub &'static str);

create_impl!(PrimitiveConversion; |self, { prereqs, arguments, .. }| {
    let kind = self.0;
    let value = &arguments[0];

    Stream::prereq(prereqs, |stream| {
        note!(stream, "local text = {value};");
    });

    format!("if type(text) == \"{kind}\" then text else nil")
});

pub struct UnOp(pub &'static str);

create_impl!(UnOp; |self, { arguments, .. }| {
    let rhs = &arguments[0];
    let op = self.0;

    format!("{op}{rhs}")
});

pub struct BinOp(pub &'static str);

create_impl!(BinOp; |self, { arguments, .. }| {
    let lhs = &arguments[0];
    let rhs = &arguments[1];
    let op = self.0;

    format!("{lhs} {op} {rhs}")
});

macro_rules! create_impl {
	($name:ident; |$self:ident, $render:tt| $block:block) => {
		impl MemberImplementation for $name {
			fn ffi_prefix(&self) -> &'static str {
				stringify!($name)
			}

			fn render(&$self, RenderData $render: RenderData) -> String {
				$block
			}
		}
	};
}

use create_impl;
