use core::panic;

use crate::codegen::{
    constants::rust::{EXCLUSIVE_INSTANCE, ROBLOX_CREATABLE, RUST_OPTION, RUST_STR},
    stream::{note, pull, push, Stream},
    structs::{
        Class, ClassEventMember, ClassFunctionMember, ClassFunctionParameter, ClassMember,
        ClassPropertyMember, Dump, PrimitiveKind, ValueType,
    },
};

use super::{
    dyn_fn_extern_name, event_extern_name, get_prop_extern_name, raw_name, set_prop_extern_name,
    to_snake,
};

fn generate_ffi_function_parameters(
    mut output: Vec<String>,
    parameters: &Vec<ClassFunctionParameter>,
) -> String {
    for parameter in parameters {
        output.push(format!(
            "p_{}: {}",
            &parameter.name,
            parameter.value_type.ffi_input_type()
        ));
    }
    output.join(", ")
}

fn generate_extern(writer: &mut Stream, dump: &Dump) {
    note!(writer, "#[allow(dead_code, improper_ctypes)]");
    push!(writer, "extern \"C\" {{");
    note!(writer, "fn get_game() -> u32;");
    for class in &dump.classes {
        for member in &class.members {
            match member {
                ClassMember::Property(property) => {
                    note!(
                        writer,
                        "fn {}(instance: u32) -> {};",
                        get_prop_extern_name(class, property),
                        property.value_type.ffi_output_type(),
                    );
                    note!(
                        writer,
                        "fn {}(instance: u32, value: {});",
                        set_prop_extern_name(class, property),
                        property.value_type.ffi_input_type(),
                    );
                }
                ClassMember::Event(event) => {
                    note!(
                        writer,
                        "fn {}(instance: u32, callback: Box<dyn Fn({})>) -> u32;",
                        event_extern_name(class, event),
                        &event
                            .parameters
                            .iter()
                            .map(|v| v.value_type.ffi_output_type())
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                ClassMember::Function(function) => {
                    note!(
                        writer,
                        "fn {}({}){};",
                        dyn_fn_extern_name(class, function),
                        generate_ffi_function_parameters(
                            vec!["instance: u32".to_string()],
                            &function.parameters
                        ),
                        match &function.return_type {
                            ValueType::Primitive(PrimitiveKind::Void) => "".to_string(),
                            value_type => format!(" -> {}", value_type.ffi_output_type()),
                        }
                    );
                }
                _ => continue,
            }
        }
    }
    pull!(writer, "}}");
}

fn generate_const_text(writer: &mut Stream) {
    note!(writer, "{}", RUST_OPTION);
    note!(writer, "{}", RUST_STR);
    note!(writer, "{}", ROBLOX_CREATABLE);
    note!(writer, "{}", EXCLUSIVE_INSTANCE);
}

fn generate_class_event(
    writer: &mut Stream,
    _dump: &Dump,
    class: &Class,
    event: &ClassEventMember,
) {
    push!(
        writer,
        "pub fn on_{}<F: Fn({}) + 'static>(&self, callback: F) -> RbxScriptConnection {{",
        to_snake(&event.name),
        &event
            .parameters
            .iter()
            .map(|v| v.value_type.rust_output_type())
            .collect::<Vec<String>>()
            .join(", ")
    );
    push!(writer, "unsafe {{");
    push!(
        writer,
        "RbxScriptConnection({}(self.to_ptr(), Box::new(move |{}| {{",
        event_extern_name(class, event),
        &event
            .parameters
            .iter()
            .map(|v| format!("p_{}", to_snake(&v.name)))
            .collect::<Vec<String>>()
            .join(", ")
    );
    note!(
        writer,
        "callback({})",
        &event
            .parameters
            .iter()
            .map(|v| convert_ffi_output_to_rust(&v.value_type, &format!("p_{}", to_snake(&v.name))))
            .collect::<Vec<String>>()
            .join(", ")
    );
    pull!(writer, "}})))");
    pull!(writer, "}}");
    pull!(writer, "}}");
}

fn generate_class_function(
    writer: &mut Stream,
    _dump: &Dump,
    class: &Class,
    function: &ClassFunctionMember,
) {
    let mut named_parameter = function
        .parameters
        .iter()
        .map(|v| {
            format!(
                "p_{}: {}",
                to_snake(&v.name),
                v.value_type.rust_input_type()
            )
        })
        .collect::<Vec<String>>();

    let mut parameter_expressions = function
        .parameters
        .iter()
        .map(|v| convert_rust_input_to_ffi(&v.value_type, &format!("p_{}", to_snake(&v.name))))
        .collect::<Vec<String>>();

    named_parameter.insert(0, "&self".to_string());
    parameter_expressions.insert(0, "self.to_ptr()".to_string());

    push!(
        writer,
        "pub fn {}({}){} {{",
        to_snake(&function.name),
        named_parameter.join(", "),
        match &function.return_type {
            ValueType::Primitive(PrimitiveKind::Void) => "".to_string(),
            value_type => format!(" -> {}", value_type.rust_output_type()),
        }
    );
    note!(
        writer,
        "unsafe {{ {} }}",
        convert_ffi_output_to_rust(
            &function.return_type,
            &format!(
                "{}({})",
                dyn_fn_extern_name(class, function),
                parameter_expressions.join(", ")
            )
        )
    );
    pull!(writer, "}}");
}

fn convert_rust_input_to_ffi(value_type: &ValueType, value: &str) -> String {
    match value_type {
        ValueType::Class(_) | ValueType::DataType(_) => format!("{value}.to_ptr()"),
        ValueType::Primitive(PrimitiveKind::String) => format!("{value}.into()"),
        ValueType::Optional(value_type) => format!(
            "{value}.map(|value| {}).into()",
            convert_rust_input_to_ffi(value_type, "value")
        ),
        ValueType::Primitive(_) => value.to_string(),
        _ => panic!("Unhandled value type"),
    }
}

fn convert_ffi_output_to_rust(value_type: &ValueType, value: &str) -> String {
    match value_type {
        ValueType::Class(kind) => format!("{kind}({value})"),
        ValueType::DataType(kind) => format!("{kind:?}({value})"),
        ValueType::Primitive(PrimitiveKind::String) => format!("{value}.into()"),
        ValueType::Primitive(_) => value.to_string(),
        ValueType::Optional(value_type) => format!(
            "Option::from({value}).map(|value| {})",
            convert_ffi_output_to_rust(value_type, "value")
        ),
        _ => panic!("Unhandled value type"),
    }
}

fn generate_class_property(
    writer: &mut Stream,
    _dump: &Dump,
    class: &Class,
    property: &ClassPropertyMember,
) {
    let property_name = raw_name(&property.name);
    let getter_name = get_prop_extern_name(class, property);
    let setter_name = set_prop_extern_name(class, property);
    let input_type = property.value_type.rust_input_type();
    let output_type = property.value_type.rust_output_type();

    push!(writer, "pub fn {property_name}(&self) -> {output_type} {{",);
    note!(
        writer,
        "{}",
        convert_ffi_output_to_rust(
            &property.value_type,
            &format!("unsafe {{ {getter_name}(self.to_ptr()) }}"),
        )
    );
    pull!(writer, "}}");

    if !property.tags.contains("ReadOnly") {
        push!(
            writer,
            "pub fn set_{}(&self, value: {input_type}) {{",
            to_snake(&property.name)
        );
        note!(
            writer,
            "unsafe {{ {setter_name}(self.to_ptr(), {}) }}",
            convert_rust_input_to_ffi(&property.value_type, "value")
        );
        pull!(writer, "}}");
    }
}

fn generate_class_member(writer: &mut Stream, dump: &Dump, class: &Class, member: &ClassMember) {
    match member {
        ClassMember::Event(event) => generate_class_event(writer, dump, class, event),
        ClassMember::Function(function) => generate_class_function(writer, dump, class, function),
        ClassMember::Property(property) => generate_class_property(writer, dump, class, property),
        ClassMember::Callback(_) => {}
    }
}

fn generate_custom_impl_macro(writer: &mut Stream, dump: &Dump, class: &Class) {
    if class.name == "Instance" {
        note!(writer, "#[repr(transparent)]");
        note!(writer, "pub struct $name(u32);");

        push!(writer, "impl Clone for $name {{");
        push!(writer, "fn clone(&self) -> Self {{");
        note!(writer, "unsafe {{ Self(clone_pointer(self.to_ptr())) }}");
        pull!(writer, "}}");
        pull!(writer, "}}");

        push!(writer, "impl Drop for $name {{");
        push!(writer, "fn drop(&mut self) {{");
        note!(writer, "unsafe {{ drop_pointer(self.to_ptr()) }}");
        pull!(writer, "}}");
        pull!(writer, "}}");
    } else if dump
        .parent(class)
        .map_or(false, |parent| parent == dump.class("Instance"))
    {
        note!(writer, "impl_instance_exclusive!($name);");
    } else if let Some(superclass) = &class.superclass {
        note!(writer, "impl_{}!($name);", to_snake(superclass));
    }

    if let Some(superclass) = &class.superclass {
        if class.name != "Instance" {
            push!(writer, "impl From<$name> for {} {{", superclass);
            push!(writer, "fn from(value: $name) -> {} {{", superclass);
            note!(
                writer,
                "unsafe {{ std::mem::transmute::<_, {}>(value) }}",
                superclass
            );
            pull!(writer, "}}");
            pull!(writer, "}}");
        }
    }
}

fn generate_custom_impl(writer: &mut Stream, class: &Class) {
    if class.tags.contains("Service") {
        push!(writer, "pub fn instance() -> Self {{");
        note!(
            writer,
            "unsafe {{ std::mem::transmute::<_, Self>(DataModel::instance().get_service(stringify!($name)).unwrap()) }}"
        );
        pull!(writer, "}}");
    }

    if class.name == "DataModel" {
        push!(writer, "pub fn instance() -> Self {{");
        note!(writer, "Self(unsafe {{ get_game() }})");
        pull!(writer, "}}");
    }

    if class.name == "Instance" {
        push!(writer, "fn to_ptr(&self) -> u32 {{");
        note!(writer, "self.0");
        pull!(writer, "}}");

        push!(writer, "pub fn downcast<I: From<$name>>(&self) -> I {{");
        note!(writer, "self.clone().into()");
        pull!(writer, "}}");
    }
}

/// Generates the impl macros for all classes.
fn generate_impl_macros(writer: &mut Stream, dump: &Dump) {
    for class in &dump.classes {
        push!(writer, "macro_rules! impl_{} {{", to_snake(&class.name));
        push!(writer, "($name:ident) => {{");
        generate_custom_impl_macro(writer, dump, class);
        push!(writer, "impl $name {{");
        generate_custom_impl(writer, class);
        for member in &class.members {
            generate_class_member(writer, dump, class, member);
        }
        pull!(writer, "}}");
        pull!(writer, "}}");
        pull!(writer, "}}");
    }
}

/// Creates the structs
fn generate_impl(writer: &mut Stream, dump: &Dump) {
    for class in &dump.classes {
        note!(writer, "impl_{}!({});", to_snake(&class.name), &class.name);
    }

    note!(
        writer,
        "creatable!({});",
        dump.classes
            .iter()
            .filter(|class| !class.tags.contains("NotCreatable"))
            .map(|class| class.name.clone())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn generate(dump: &Dump) -> String {
    let mut writer = Stream::new();
    note!(
        writer,
        "// Generated by wlausam-bindings at {}",
        chrono::offset::Local::now().format("%A, %B %Y %r")
    );
    note!(writer, "pub use super::*;");
    generate_extern(&mut writer, dump);
    generate_const_text(&mut writer);
    generate_impl_macros(&mut writer, dump);
    generate_impl(&mut writer, dump);
    writer.stream
}
