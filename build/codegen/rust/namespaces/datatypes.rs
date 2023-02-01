use crate::codegen::{
    rust::{
        constants::DATATYPE_IMPL_MACRO, conversion::get_owned_rust_type, generate_body,
        generate_member,
    },
    stream::{note, pull, push, Stream},
    structs::{DeclarationContext, Member, Namespace},
};

fn generate_operator(output: &mut Vec<String>, namespace: &Namespace, member: &Member, op: &str) {
    let class = &namespace.name;
    let func = &member.name;

    let output_type = get_owned_rust_type(&member.outputs[0]);
    let body = generate_body(namespace, member);

    match &member.inputs[1..] {
        // unary operators
        [] => Stream::prereq(output, |stream| {
            push!(stream, "impl std::ops::{op} for {class} {{");
            note!(stream, "type Output = {output_type};");
            push!(stream, "fn {func}(self) -> Self::Output {{");
            note!(stream, "unsafe {{ {body} }}");
            pull!(stream, "}}");
            pull!(stream, "}}");
        }),
        // binary operators
        [rhs] => {
            let name = &rhs.name;
            let kind = get_owned_rust_type(&rhs.kind);
            Stream::prereq(output, |stream| {
                push!(stream, "impl std::ops::{op}<{kind}> for {class} {{");
                note!(stream, "type Output = {output_type};");
                push!(stream, "fn {func}(self, {name}: {kind}) -> Self::Output {{");
                note!(stream, "unsafe {{ {body} }}");
                pull!(stream, "}}");
                pull!(stream, "}}");
            })
        }
        other => panic!("unsupported operator arity {}", other.len()),
    }
}

pub fn generate_datatypes(output: &mut Vec<String>, namespaces: &[&Namespace]) {
    output.push(DATATYPE_IMPL_MACRO.to_string());

    for namespace in namespaces {
        let name = &namespace.name;

        let mut operators = Vec::new();
        let mut members = Vec::new();
        for member in &namespace.members {
            if let Some(operator) = member.flags.operator {
                generate_operator(&mut operators, namespace, member, operator);
            } else {
                generate_member(
                    &mut members,
                    namespace,
                    member,
                    DeclarationContext::Function,
                );
            }
        }

        Stream::prereq(output, |stream| {
            note!(stream, "impl_data_type!({name});");

            if !members.is_empty() {
                push!(stream, "impl {name} {{");
                note!(stream in members);
                pull!(stream, "}}");
            }

            note!(stream in operators);
        })
    }
}
