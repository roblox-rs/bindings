use crate::codegen::{
    stream::{note, pull, push, Stream},
    structs::{CodegenKind, Parameter},
    UniqueIds,
};

use super::{load_memory, store_memory};

fn convert_vec(
    identifier: &str,
    ids: &UniqueIds,
    kind: &CodegenKind,
    length: String,
    prereqs: &mut Vec<String>,
) -> Vec<String> {
    let param_vec = ids.next_names(&[identifier, "vec"]);
    let vec_value = ids.next_names(&[identifier, "vec_value"]);
    let layout = kind.layout();
    let size = layout.size();

    let mut luau_prereqs = Vec::new();
    let results = convert_luau_to_rust(ids, &vec_value, kind, &mut luau_prereqs, false);
    let stores = store_memory(layout, &format!("{param_vec} + (i - 1) * {size}"), &results);

    Stream::prereq(prereqs, |stream| {
        note!(stream, "local {param_vec} = allocVec({length} * {size})");
        push!(stream, "for i = 1, {length} do");
        note!(stream, "local {vec_value} = {identifier}[i];");
        note!(stream in luau_prereqs);
        note!(stream, "{stores}");
        pull!(stream, "end");
    });

    vec![param_vec, length]
}

pub fn convert_luau_to_rust(
    ids: &UniqueIds,
    identifier: &str,
    kind: &CodegenKind,
    prereqs: &mut Vec<String>,
    variadic: bool,
) -> Vec<String> {
    match &kind {
        CodegenKind::Function(_, _) => panic!(),
        CodegenKind::Optional(kind) => {
            let param_optional = ids.next_names(&[identifier, "optional"]);
            let width = kind.width();
            let mut inputs = vec![format!("{param_optional}")];
            for i in 0..width {
                inputs.push(format!("{param_optional}_{i}"));
            }

            let mut luau_prereqs = Vec::new();
            let result = convert_luau_to_rust(ids, identifier, kind, &mut luau_prereqs, false);

            Stream::prereq(prereqs, |stream| {
                let input_names = inputs.join(", ");
                let input_values = vec!["0"; inputs.len()].join(", ");
                let formatted_result = result.join(", ");

                note!(stream, "local {input_names} = {input_values};");

                push!(stream, "if ({identifier} ~= nil) then");
                note!(stream in luau_prereqs);
                note!(stream, "{input_names} = 1, {formatted_result};");
                pull!(stream, "end");
            });

            inputs
        }
        CodegenKind::Tuple(kind) => {
            let param_tuple_len = ids.next_names(&[identifier, "len"]);
            let name = if variadic {
                let param_tuple = ids.next_names(&[identifier, "tuple"]);

                Stream::prereq(prereqs, |stream| {
                    note!(stream, "local {param_tuple} = {{ ... }};");
                    note!(stream, "local {param_tuple_len} = select(\"#\", ...);")
                });

                param_tuple
            } else {
                Stream::prereq(prereqs, |stream| {
                    note!(stream, "local {param_tuple_len} = #{identifier};")
                });

                identifier.to_string()
            };

            convert_vec(&name, ids, kind, param_tuple_len, prereqs)
        }
        CodegenKind::Vec(kind) => {
            let param_tuple_len = ids.next_names(&[identifier, "len"]);

            Stream::prereq(prereqs, |stream| {
                note!(stream, "local {param_tuple_len} = #{identifier};")
            });

            convert_vec(identifier, ids, kind, param_tuple_len, prereqs)
        }
        CodegenKind::Instance(_) => vec![format!("createPointer({identifier})")],
        CodegenKind::DataType(_) => vec![format!("createPointer({identifier})")],
        CodegenKind::Unknown => vec![format!("createPointer({identifier})")],
        CodegenKind::Enum(_) => vec![identifier.to_string()],
        CodegenKind::String => {
            let param_len = ids.next_names(&[identifier, "len"]);
            let param_content = ids.next_names(&[identifier, "content"]);

            Stream::prereq(prereqs, |stream| {
                note!(stream, "local {param_len} = #{identifier};");
                note!(stream, "local {param_content} = allocString({param_len});");
                push!(stream, "for i = 1, {param_len} do");
                note!(stream, "storeU8(memory, {param_content} + (i - 1), string.byte({identifier}:sub(i, i)));");
                pull!(stream, "end");
            });

            vec![param_content, param_len]
        }
        CodegenKind::Number => vec![identifier.to_string()],
        CodegenKind::Bool => vec![format!("{identifier} and 1 or 0")],
        CodegenKind::Void => vec![],
    }
}

fn get_lua_parameter_name(parameter: &Parameter) -> String {
    if let CodegenKind::Tuple(_) = &parameter.kind {
        "...".to_string()
    } else {
        parameter.name.clone()
    }
}

pub fn convert_rust_to_luau(
    ids: &UniqueIds,
    parameter: &Parameter,
    arguments: &[String],
    prereqs: &mut Vec<String>,
) -> String {
    match &parameter.kind {
        CodegenKind::Function(parameters, output) => {
            let layout = output.layout();
            let parameter_names: Vec<_> = parameters.iter().map(get_lua_parameter_name).collect();

            let mut luau_prereqs = Vec::new();
            let mut arguments = vec![arguments[0].clone(), arguments[1].clone()];
            arguments.extend(parameters.iter().flat_map(|v| {
                convert_luau_to_rust(ids, &v.name, &v.kind, &mut luau_prereqs, true)
            }));

            let mut result_prereqs = Vec::new();
            let expression = format!("invokeFunction({})", arguments.join(", "));
            if layout.width() == 0 {
                Stream::prereq(&mut result_prereqs, |stream| note!(stream, "{expression}"));
            } else {
                let mut inputs = Vec::new();
                for i in 0..layout.width() {
                    inputs.push(ids.next_names(&["result", &i.to_string()]));
                }

                let mut luau_prereqs = Vec::new();
                let result = convert_rust_to_luau(
                    ids,
                    &parameter.with_kind(output),
                    &inputs,
                    &mut luau_prereqs,
                );

                Stream::prereq(&mut result_prereqs, |stream| {
                    let input_names = inputs.join(", ");

                    note!(stream, "local {input_names} = {expression};");
                    note!(stream in luau_prereqs);
                    note!(stream, "return {result};");
                })
            }

            Stream::expression(|stream| {
                let formatted_parameters = parameter_names.join(", ");

                push!(stream, "function({formatted_parameters})");
                note!(stream in luau_prereqs);
                note!(stream in result_prereqs);
                pull!(stream, "end");
            })
        }
        CodegenKind::Tuple(kind) => {
            let result = convert_rust_to_luau(
                ids,
                &parameter.with_kind(&CodegenKind::Vec(kind.clone())),
                arguments,
                prereqs,
            );

            format!("unpack({result}, 1, {})", arguments[1])
        }
        CodegenKind::Optional(codegen_type) => {
            let param_optional = ids.next_names(&[&parameter.name, "optional"]);
            let mut optional_prereqs = Vec::new();
            let input = convert_rust_to_luau(
                ids,
                &parameter.with_kind(codegen_type),
                &arguments[1..],
                &mut optional_prereqs,
            );

            Stream::prereq(prereqs, |stream| {
                note!(stream, "local {param_optional};");
                push!(stream, "if ({} == 1) then", arguments[0]);
                note!(stream in optional_prereqs);
                note!(stream, "{param_optional} = {};", input);
                pull!(stream, "end");
            });

            param_optional.clone()
        }
        CodegenKind::Vec(kind) => {
            let param_vec = ids.next_names(&[&parameter.name, "vec"]);
            let vec_addr = &arguments[0];
            let vec_len = &arguments[1];

            let layout = kind.layout();
            let size = layout.size();
            let vec_element = ids.next_names(&["vecElement"]);
            let outputs: Vec<_> = (0..layout.size())
                .map(|v| format!("{vec_element}_{v}"))
                .collect();

            let loads = load_memory(layout, &format!("{vec_addr} + (i - 1) * {size}"), &outputs);

            let mut vec_prereqs = Vec::new();
            let input =
                convert_rust_to_luau(ids, &parameter.with_kind(kind), &outputs, &mut vec_prereqs);

            Stream::prereq(prereqs, |stream| {
                note!(stream, "local {param_vec} = table.create({vec_len});");
                push!(stream, "for i = 1, {vec_len} do");
                note!(stream, "{loads}");
                note!(stream in vec_prereqs);
                note!(stream, "{param_vec}[i] = {input};");
                pull!(stream, "end");
            });

            param_vec.to_string()
        }
        CodegenKind::String => format!("loadString(memory, {}, {})", arguments[0], arguments[1]),
        CodegenKind::Instance(_) => format!("getPointer({})", arguments[0]),
        CodegenKind::DataType(_) => format!("getPointer({})", arguments[0]),
        CodegenKind::Unknown => format!("getPointer({})", arguments[0]),
        CodegenKind::Enum(_) => arguments[0].clone(),
        CodegenKind::Number => arguments[0].clone(),
        CodegenKind::Bool => format!("{} == 1", arguments[0]),
        CodegenKind::Void => "nil".to_string(),
    }
}
