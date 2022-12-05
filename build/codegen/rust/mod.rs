use crate::{
    codegen::{
        rust::conversion::owned_ffi_to_rust,
        stream::{note, pull, write_to},
    },
    dump::types::Enum,
};

use self::{
    constants::{
        EXCLUSIVE_INSTANCE, ROBLOX_CREATABLE, RUST_OPTION, RUST_ROBLOX_ENUM_MACRO, RUST_SLICE,
        RUST_STRING, RUST_VEC,
    },
    conversion::{
        borrowed_rust_to_ffi, get_borrowed_ffi_type, get_borrowed_rust_type, get_owned_ffi_type,
        get_owned_rust_type,
    },
    namespaces::{
        datatypes::generate_datatypes, functions::generate_functions, instances::generate_instances,
    },
};

use super::{
    stream::{push, Stream},
    structs::{CodegenKind, DeclarationContext, Member, Namespace, NamespaceKind, Parameter},
};

mod constants;
mod conversion;
mod namespaces;

fn raw_name(str: &str) -> String {
    if matches!(str, "type" | "move" | "loop") {
        format!("r#{str}")
    } else {
        str.to_string()
    }
}

fn raw_ffi_name(str: &str) -> String {
    if matches!(str, "self") {
        format!("p_{str}")
    } else {
        raw_name(str)
    }
}

fn generate_body(namespace: &Namespace, member: &Member) -> String {
    let ffi_name = namespace.ffi_name(member);
    let inputs: Vec<_> = member
        .inputs
        .iter()
        .map(|v| borrowed_rust_to_ffi(&raw_name(&v.name), &v.kind))
        .collect();

    Stream::expression(|stream| {
        let inputs = inputs.join(", ");
        let has_output = !member.outputs.is_empty();

        let expression = format!("{ffi_name}({inputs})");
        if has_output {
            let transformation = owned_ffi_to_rust(&expression, &member.outputs[0]);

            note!(stream, "{transformation}");
        } else {
            note!(stream, "{expression}");
        }
    })
}

type DeclarationFunctions = (fn(&CodegenKind) -> String, fn(&CodegenKind) -> String);

fn get_declaration_functions(is_extern: bool) -> DeclarationFunctions {
    if is_extern {
        (get_owned_ffi_type, get_borrowed_ffi_type)
    } else {
        (get_owned_rust_type, get_borrowed_rust_type)
    }
}

fn generate_member_declaration(member: &Member, name: &str, context: DeclarationContext) -> String {
    let is_extern = matches!(context, DeclarationContext::Extern);
    let (get_owned_type, get_borrowed_type) = get_declaration_functions(is_extern);

    let name = raw_name(name);
    let outputs: Vec<_> = member.outputs.iter().map(get_owned_type).collect();
    let mut parameters = Vec::new();

    for Parameter { name, kind } in &member.inputs {
        let name = if is_extern {
            raw_ffi_name(name)
        } else {
            raw_name(name)
        };

        let parameter_type = get_borrowed_type(kind);
        if name == "self" {
            parameters.push("&self".to_string());
        } else if let CodegenKind::Function(inputs, output) = &kind {
            let is_definition = matches!(context, DeclarationContext::Trait) || is_extern;

            if inputs.is_empty() && matches!(output.as_ref(), CodegenKind::Void) || is_definition {
                parameters.push(format!("{name}: {parameter_type}"))
            } else {
                parameters.push(format!("mut {name}: {parameter_type}"));
            }
        } else {
            parameters.push(format!("{name}: {parameter_type}"));
        }
    }

    Stream::expression(|stream| {
        let parameters = parameters.join(", ");
        let return_types = outputs.join(", ");

        if matches!(context, DeclarationContext::Function) {
            write_to!(stream, "pub ");
        }

        write_to!(stream, "fn {name}({parameters})");

        if outputs.len() > 1 {
            write_to!(stream, " -> ({return_types})");
        } else if !outputs.is_empty() {
            write_to!(stream, " -> {return_types}");
        };
    })
}

fn generate_member(
    out: &mut Vec<String>,
    namespace: &Namespace,
    member: &Member,
    context: DeclarationContext,
) {
    let declaration = generate_member_declaration(member, &member.name, context);
    let body = generate_body(namespace, member);

    let is_declaration = matches!(
        context,
        DeclarationContext::Trait | DeclarationContext::Function
    );

    Stream::prereq(out, |stream| {
        if member.flags.deprecated && is_declaration {
            note!(stream, "#[deprecated]");
        }

        push!(stream, "{declaration} {{");
        push!(stream, "unsafe {{");
        note!(stream, "{body}");
        pull!(stream, "}}");
        pull!(stream, "}}");
    })
}

fn generate_extern(namespaces: &[Namespace]) -> String {
    let mut definitions = Vec::new();

    for namespace in namespaces {
        for member in &namespace.members {
            let name = namespace.ffi_name(member);
            let declaration =
                generate_member_declaration(member, &name, DeclarationContext::Extern);

            Stream::prereq(&mut definitions, |stream| {
                note!(stream, "{declaration};");
            })
        }
    }

    Stream::expression(|stream| {
        note!(stream, "#[allow(clippy::all)]");
        note!(stream, "#[allow(improper_ctypes)]");
        push!(stream, "extern \"C\" {{");
        note!(stream in definitions);
        pull!(stream, "}}");
    })
}

fn generate_enums(output: &mut Vec<String>, enums: &[Enum]) {
    Stream::prereq(output, |stream| {
        push!(stream, "pub mod enums {{");

        for roblox_enum in enums {
            let enum_name = &roblox_enum.name;
            let mut members = Vec::new();

            for enum_item in &roblox_enum.items {
                let value = enum_item.value;
                let sanitized_name = if enum_item.name == "Self" {
                    "VariantSelf"
                } else {
                    &enum_item.name
                };

                members.push(format!("{sanitized_name} = {value},"));
            }

            push!(stream, "roblox_enum!({enum_name}; {{");
            note!(stream in members);
            pull!(stream, "}});");
        }

        pull!(stream, "}}");
    })
}

fn generate_constant(output: &mut Vec<String>) {
    output.extend(
        [
            RUST_VEC,
            RUST_OPTION,
            RUST_STRING,
            RUST_SLICE,
            ROBLOX_CREATABLE,
            RUST_ROBLOX_ENUM_MACRO,
            EXCLUSIVE_INSTANCE,
        ]
        .map(ToString::to_string),
    );
}

pub fn generate(namespaces: &[Namespace], enums: &[Enum]) -> String {
    let mut output = Vec::new();
    output.push(generate_extern(namespaces));
    output.push("use super::*;".to_string());
    generate_constant(&mut output);
    generate_enums(&mut output, enums);

    let mut instances = Vec::new();
    let mut functions = Vec::new();
    let mut datatypes = Vec::new();

    for namespace in namespaces {
        match namespace.kind {
            NamespaceKind::Instance(_) => instances.push(namespace),
            NamespaceKind::DataType => datatypes.push(namespace),
            NamespaceKind::Functions => functions.push(namespace),
        }
    }

    generate_instances(&mut output, &instances);
    generate_datatypes(&mut output, &datatypes);
    generate_functions(&mut output, &functions);

    output.join("\n")
}
