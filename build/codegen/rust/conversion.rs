use crate::codegen::{
    stream::{note, pull, push, Stream},
    structs::CodegenKind,
};

use super::raw_name;

// The bindings pass these types to the Luau bindings
pub fn get_borrowed_ffi_type(kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(inputs, result) => {
            let return_type = get_owned_ffi_type(result);
            let parameters = inputs
                .iter()
                .map(|parameter| get_owned_ffi_type(&parameter.kind))
                .collect::<Vec<_>>()
                .join(", ");

            format!("Box<dyn FnMut({parameters}) -> {return_type}>")
        }
        CodegenKind::Optional(kind) => format!("RustOption<{}>", get_borrowed_ffi_type(kind)),
        CodegenKind::Tuple(kind) => format!("RustSlice<{}>", get_borrowed_ffi_type(kind)),
        CodegenKind::Vec(kind) => format!("RustSlice<{}>", get_borrowed_ffi_type(kind)),
        CodegenKind::Instance(_) => "u32".to_string(),
        CodegenKind::DataType(_) => "u32".to_string(),
        CodegenKind::Enum(kind) => format!("&enums::{kind}"),
        CodegenKind::Unknown => "u32".to_string(),
        CodegenKind::String => "RustSlice<u8>".to_string(),
        CodegenKind::Number => "f64".to_string(),
        CodegenKind::Bool => "bool".to_string(),
        CodegenKind::Void => "()".to_string(),
    }
}

// The bindings receive these types from the Lua bindings
pub fn get_owned_ffi_type(kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(_, _) => unimplemented!(),
        CodegenKind::Optional(kind) => format!("RustOption<{}>", get_owned_ffi_type(kind)),
        CodegenKind::Tuple(kind) => format!("RustVec<{}>", get_owned_ffi_type(kind)),
        CodegenKind::Vec(kind) => format!("RustVec<{}>", get_owned_ffi_type(kind)),
        CodegenKind::Instance(_) => "u32".to_string(),
        CodegenKind::DataType(kind) => kind.to_string(),
        CodegenKind::Enum(kind) => format!("enums::{kind}"),
        CodegenKind::Unknown => "u32".to_string(),
        CodegenKind::String => "RustString".to_string(),
        CodegenKind::Number => "f64".to_string(),
        CodegenKind::Bool => "bool".to_string(),
        CodegenKind::Void => "()".to_string(),
    }
}

// Users pass these types to the Rust bindings
pub fn get_borrowed_rust_type(kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(inputs, result) => {
            let return_type = get_owned_rust_type(result);
            let parameters = inputs
                .iter()
                .map(|parameter| get_owned_rust_type(&parameter.kind))
                .collect::<Vec<_>>()
                .join(", ");

            format!("impl FnMut({parameters}) -> {return_type} + 'static")
        }
        CodegenKind::Optional(kind) => format!("&Option<{}>", get_borrowed_rust_type(kind)),
        CodegenKind::Tuple(kind) => format!("&[{}]", get_borrowed_rust_type(kind)),
        CodegenKind::Vec(kind) => format!("&[{}]", get_borrowed_rust_type(kind)),
        CodegenKind::Instance(kind) => format!("&{kind}"),
        CodegenKind::DataType(kind) => format!("&{kind}"),
        CodegenKind::Unknown => "&LuaValue".to_string(),
        CodegenKind::Enum(kind) => format!("&enums::{kind}"),
        CodegenKind::String => "&str".to_string(),
        CodegenKind::Number => "f64".to_string(),
        CodegenKind::Bool => "bool".to_string(),
        CodegenKind::Void => "()".to_string(),
    }
}

// Users receive these types from the Rust bindings
pub fn get_owned_rust_type(kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(_, _) => unimplemented!(),
        CodegenKind::Optional(kind) => format!("Option<{}>", get_owned_rust_type(kind)),
        CodegenKind::Tuple(kind) => format!("Vec<{}>", get_owned_rust_type(kind)),
        CodegenKind::Vec(kind) => format!("Vec<{}>", get_owned_rust_type(kind)),
        CodegenKind::Instance(kind) => kind.to_string(),
        CodegenKind::DataType(kind) => kind.to_string(),
        CodegenKind::Unknown => "LuaValue".to_string(),
        CodegenKind::Enum(kind) => format!("enums::{kind}"),
        CodegenKind::String => "String".to_string(),
        CodegenKind::Number => "f64".to_string(),
        CodegenKind::Bool => "bool".to_string(),
        CodegenKind::Void => "()".to_string(),
    }
}

pub fn borrowed_rust_to_ffi(expression: &str, kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(inputs, output) => {
            if inputs.is_empty() && matches!(output.as_ref(), CodegenKind::Void) {
                return format!("Box::new({expression})");
            }

            let input_names: Vec<_> = inputs.iter().map(|v| raw_name(&v.name)).collect();
            let transformed_inputs: Vec<_> = inputs
                .iter()
                .map(|v| owned_ffi_to_rust(&raw_name(&v.name), &v.kind))
                .collect();

            Stream::expression(|stream| {
                let input_names = input_names.join(", ");
                let transformed_inputs = transformed_inputs.join(", ");
                let has_return = !matches!(output.as_ref(), CodegenKind::Void);
                let expression = format!("{expression}({transformed_inputs})");

                push!(stream, "Box::new(move |{input_names}| {{");
                if has_return {
                    let transformation = owned_rust_to_ffi(&expression, output);

                    note!(stream, "{transformation}");
                } else {
                    note!(stream, "{expression}");
                }
                pull!(stream, "}})");
            })
        }
        CodegenKind::Optional(kind) => {
            let conversion = borrowed_rust_to_ffi("value", kind);

            format!("RustOption::from({expression}.map(|value| {conversion}))")
        }
        CodegenKind::Vec(kind) | CodegenKind::Tuple(kind) => {
            let conversion = borrowed_rust_to_ffi("value", kind);

            // format!("RustSlice::from({expression}.iter().map(|value| {conversion}).collect())")
            Stream::expression(|stream| {
                push!(stream, "{{");
                note!(stream, "let mapped = {expression}.iter().map(|value| {conversion}).collect::<Vec<_>>();");
                note!(stream, "RustSlice::from(&mapped[..])");
                pull!(stream, "}}");
            })
        }
        CodegenKind::Instance(_) => format!("{expression}.0"),
        CodegenKind::DataType(_) => format!("{expression}.0"),
        CodegenKind::Enum(_) => expression.to_string(),
        CodegenKind::String => format!("RustSlice::from({expression}.as_bytes())"),
        CodegenKind::Number => expression.to_string(),
        CodegenKind::Bool => expression.to_string(),
        CodegenKind::Void => expression.to_string(),
        CodegenKind::Unknown => format!("{expression}.0"),
    }
}

pub fn owned_ffi_to_rust(expression: &str, kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(_, _) => unreachable!(),
        CodegenKind::Optional(kind) => {
            let conversion = owned_ffi_to_rust("value", kind);
            let conversion_type = get_owned_ffi_type(kind);

            // Rust struggles to infer the type for some reason,
            // so it is manually specified.
            format!("Option::<{conversion_type}>::from({expression}).map(|value| {conversion})")
        }
        CodegenKind::Vec(kind) | CodegenKind::Tuple(kind) => {
            let conversion = owned_ffi_to_rust("value", kind);

            format!(
                "Vec::from({expression}).into_iter().map(|value| {conversion}).collect::<Vec<_>>()"
            )
        }
        CodegenKind::Instance(kind) => format!("{kind}({expression})"),
        CodegenKind::DataType(_) => expression.to_string(),
        CodegenKind::Enum(_) => expression.to_string(),
        CodegenKind::String => format!("String::from({expression})"),
        CodegenKind::Number => expression.to_string(),
        CodegenKind::Bool => expression.to_string(),
        CodegenKind::Void => expression.to_string(),
        CodegenKind::Unknown => format!("LuaValue({expression})"),
    }
}

pub fn owned_rust_to_ffi(expression: &str, kind: &CodegenKind) -> String {
    match kind {
        CodegenKind::Function(_, _) => unimplemented!(),
        CodegenKind::Tuple(kind) => {
            let transformation = borrowed_rust_to_ffi("value", kind);

            format!(
                "RustVec::from({expression}.into_iter().map(|value| {transformation}).collect::<Vec<_>>())"
            )
        }
        CodegenKind::Optional(kind) => {
            let transformation = borrowed_rust_to_ffi("value", kind);

            format!("RustOption::from({expression}).map(|value| {transformation})")
        }
        CodegenKind::Vec(kind) => {
            let transformation = borrowed_rust_to_ffi("value", kind);

            format!(
                "RustVec::from({expression}.into_iter().map(|value| {transformation}).collect::<Vec<_>>())"
            )
        }
        CodegenKind::Instance(_) => format!("{expression}.0"),
        CodegenKind::DataType(_) => expression.to_string(),
        CodegenKind::Enum(_) => expression.to_string(),
        CodegenKind::String => format!("RustString::from({expression})"),
        CodegenKind::Unknown => expression.to_string(),
        CodegenKind::Number => expression.to_string(),
        CodegenKind::Bool => expression.to_string(),
        CodegenKind::Void => expression.to_string(),
    }
}
