use crate::{
    rust::generate_member,
    structs::{DeclarationContext, Namespace},
};
use convert_case::{Case, Casing};
use stream::{note, pull, push, Stream};

pub fn generate_functions(output: &mut Vec<String>, namespaces: &[&Namespace]) {
    for namespace in namespaces {
        let name = namespace.name.to_case(Case::Snake);

        let mut members = Vec::new();
        for member in &namespace.members {
            generate_member(
                &mut members,
                namespace,
                member,
                DeclarationContext::Function,
            );
        }

        Stream::prereq(output, |stream| {
            push!(stream, "pub mod {name} {{");
            note!(stream, "use super::*;");
            note!(stream in members);
            pull!(stream, "}}");
        })
    }
}
