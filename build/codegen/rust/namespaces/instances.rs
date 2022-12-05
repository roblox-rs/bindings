use convert_case::{Case, Casing};

use crate::codegen::{
    rust::{
        constants::{
            CUSTOM_IMPL_DATA_MODEL, CUSTOM_IMPL_INSTANCE, CUSTOM_IMPL_SERVICE,
            RUST_INSTANCE_CUSTOM_IMPL,
        },
        generate_member, generate_member_declaration,
    },
    stream::{note, pull, push, Stream},
    structs::{DeclarationContext, Namespace, NamespaceKind},
};

fn generate_custom_impl(output: &mut Vec<String>, namespace: &Namespace) {
    let NamespaceKind::Instance(metadata) = &namespace.kind else { unreachable!() };

    if metadata.service {
        output.push(CUSTOM_IMPL_SERVICE.to_string());
    }

    if namespace.name == "DataModel" {
        output.push(CUSTOM_IMPL_DATA_MODEL.to_string());
    }

    if namespace.name == "Instance" {
        output.push(CUSTOM_IMPL_INSTANCE.to_string());
    }
}

fn generate_traits(output: &mut Vec<String>, namespaces: &[&Namespace]) {
    for class in namespaces {
        let name = &class.name;

        Stream::prereq(output, |stream| {
            if let Some(parent) = &class.extends {
                push!(stream, "pub trait I{name}: I{parent} {{");
            } else {
                push!(stream, "pub trait I{name} {{");
            }

            for member in &class.members {
                let declaration =
                    generate_member_declaration(member, &member.name, DeclarationContext::Trait);

                if member.flags.deprecated {
                    note!(stream, "#[deprecated]");
                }

                note!(stream, "{declaration};");
            }

            pull!(stream, "}}");
        })
    }
}

fn generate_deref_impl(output: &mut Vec<String>, namespaces: &[&Namespace]) {
    for class in namespaces {
        let name = &class.name;

        if let Some(extends) = &class.extends {
            Stream::prereq(output, |stream| {
                push!(stream, "impl std::ops::Deref for {name} {{");
                note!(stream, "type Target = {extends};");
                push!(stream, "fn deref(&self) -> &Self::Target {{");
                note!(stream, "unsafe {{ std::mem::transmute(self) }}");
                pull!(stream, "}}");
                pull!(stream, "}}");
            });
        }
    }
}

pub fn generate_instances(output: &mut Vec<String>, namespaces: &[&Namespace]) {
    let mut creatable = Vec::new();

    for class in namespaces {
        let NamespaceKind::Instance(metadata) = &class.kind else { unreachable!() };
        let class_name = &class.name;
        let impl_name = class_name.to_case(Case::Snake);

        let mut members = Vec::new();
        for member in &class.members {
            generate_member(&mut members, class, member, DeclarationContext::TraitImpl);
        }

        let mut custom_impl = Vec::new();
        generate_custom_impl(&mut custom_impl, class);

        if metadata.creatable {
            creatable.push(class.name.clone());
        }

        Stream::prereq(output, |stream| {
            push!(stream, "macro_rules! impl_{impl_name} {{");
            push!(stream, "($name:ident) => {{");

            if class.name == "Instance" {
                note!(stream, "{RUST_INSTANCE_CUSTOM_IMPL}");
            }

            if let Some(extends) = &class.extends {
                let impl_extends = if extends == "Instance" {
                    "instance_exclusive".to_string()
                } else {
                    extends.to_case(Case::Snake)
                };

                note!(stream, "impl_{impl_extends}!($name);");

                push!(stream, "impl From<$name> for {extends} {{");
                push!(stream, "fn from(value: $name) -> {extends} {{");
                push!(stream, "unsafe {{");
                note!(stream, "std::mem::transmute::<_, {extends}>(value)");
                pull!(stream, "}}");
                pull!(stream, "}}");
                pull!(stream, "}}");
            }

            push!(stream, "impl I{class_name} for $name {{");
            note!(stream in members);
            pull!(stream, "}}");

            if !custom_impl.is_empty() {
                push!(stream, "impl $name {{");
                note!(stream in custom_impl);
                pull!(stream, "}}");
            }

            pull!(stream, "}}");
            pull!(stream, "}}");
        })
    }

    generate_traits(output, namespaces);
    generate_deref_impl(output, namespaces);

    Stream::prereq(output, |stream| {
        let creatable = creatable.join(" ");

        for Namespace { name, .. } in namespaces {
            let impl_name = name.to_case(Case::Snake);

            note!(stream, "impl_{impl_name}!({name});");
        }

        note!(stream, "creatable!({creatable});");
    })
}
