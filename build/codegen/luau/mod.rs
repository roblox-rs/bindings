pub mod constants;
mod conversion;

use crate::codegen::{
    luau::constants::{LUAU_CREATE_CONNECTION, LUAU_DISCONNECT_CONNECTION, LUA_VALUE_NUMBER_TYPES},
    stream::{note, pull, push},
    UniqueIds,
};

use self::conversion::{convert_luau_to_rust, convert_rust_to_luau};

use super::{
    stream::Stream,
    structs::{CodegenKind, Member, Namespace, RenderData, Representation, TypeLayout},
};

fn store_memory(layout: TypeLayout, address: &str, inputs: &[String]) -> String {
    Stream::expression(|stream| {
        for (i, (offset, rep)) in layout.iter().enumerate() {
            let input = &inputs[i];
            let name = match rep {
                Representation::Pointer => "storeU32",
                Representation::Float => "storeFloat",
                Representation::Byte => "storeU8",
            };

            note!(stream, "{name}(memory, {address} + {offset}, {input});")
        }
    })
}

fn store_memory_or_return(layout: TypeLayout, address: &str, inputs: &[String]) -> String {
    if !layout.needs_spill() {
        let results = inputs.join(", ");

        format!("return {results};")
    } else {
        store_memory(layout, address, inputs)
    }
}

fn load_memory(layout: TypeLayout, address: &str, outputs: &[String]) -> String {
    Stream::expression(|stream| {
        for (i, (offset, rep)) in layout.iter().enumerate() {
            let output = &outputs[i];
            let name = match rep {
                Representation::Pointer => "loadU32",
                Representation::Float => "loadFloat",
                Representation::Byte => "loadU8",
            };

            note!(
                stream,
                "local {output} = {name}(memory, {address} + {offset})"
            );
        }
    })
}

fn render_member_effect(
    namespace: &Namespace,
    member: &Member,
    arguments: &[String],
    parameters: &[Vec<String>],
    prereqs: &mut Vec<String>,
) -> String {
    member.implementation.render(RenderData {
        namespace_name: &namespace.name,
        member_name: &member.api_name,

        namespace,
        member,
        arguments,
        parameters,
        prereqs,
    })
}

fn generate_member(out: &mut Vec<String>, namespace: &Namespace, member: &Member) {
    let ids = UniqueIds::new();
    let ffi_name = namespace.ffi_name(member);
    let raw_parameters: Vec<_> = member.inputs.iter().map(|v| v.names(&ids)).collect();
    let mut parameters: Vec<_> = raw_parameters.iter().flatten().cloned().collect();

    let mut arguments_prereqs = Vec::new();
    let arguments: Vec<_> = member
        .inputs
        .iter()
        .enumerate()
        .map(|(i, v)| convert_rust_to_luau(&ids, v, &raw_parameters[i], &mut arguments_prereqs))
        .collect();

    let mut effect_prereqs = Vec::new();
    let effect = render_member_effect(
        namespace,
        member,
        &arguments,
        &raw_parameters,
        &mut effect_prereqs,
    );

    let layout = TypeLayout::from_tuple(&member.outputs);
    let effect = if layout.width() == 0 {
        effect
    } else {
        if layout.needs_spill() {
            parameters.insert(0, "output".to_string());
        }

        let mut prereqs = Vec::new();
        let names: Vec<_> = (0..member.outputs.len())
            .map(|i| ids.next_names(&["result", &i.to_string()]))
            .collect();

        if let Some(CodegenKind::Tuple(_)) = member.outputs.last() {
            let tuple_name = names.last().unwrap();

            Stream::prereq(&mut prereqs, |stream| {
                note!(stream, "local {tuple_name} = {{ {effect } }}");

                for name in &names[..names.len() - 1] {
                    note!(stream, "local {name} = table.remove({tuple_name}, 1);");
                }
            })
        } else {
            Stream::prereq(&mut prereqs, |stream| {
                let formatted_names = names.join(", ");

                note!(stream, "local {formatted_names} = {effect};");
            })
        }

        let results: Vec<_> = member
            .outputs
            .iter()
            .enumerate()
            .flat_map(|(i, kind)| convert_luau_to_rust(&ids, &names[i], kind, &mut prereqs, false))
            .collect();

        let stores = store_memory_or_return(layout, "output", &results);

        Stream::expression(|stream| {
            note!(stream in prereqs);
            note!(stream, "{stores}");
        })
    };

    Stream::prereq(out, |stream| {
        let parameters = parameters.join(", ");

        push!(stream, "function abi.ffi.{ffi_name}({parameters})");
        note!(stream in arguments_prereqs);
        note!(stream in effect_prereqs);
        note!(stream, "{effect}");
        pull!(stream, "end");
    });
}

fn generate_namespace(out: &mut Vec<String>, namespace: &Namespace) {
    for member in &namespace.members {
        generate_member(out, namespace, member);
    }
}

fn generate_abi_start() -> String {
    let paths = vec![
        ("getPointer", "util.getPointer"),
        ("createPointer", "util.createPointer"),
        ("memory", "wasm.memory_list.memory"),
        ("storeU8", "rt.store.i32_n8"),
        ("storeU32", "rt.store.i32"),
        ("storeFloat", "rt.store.f64"),
        ("loadU8", "rt.load.i32_u8"),
        ("loadU32", "rt.load.i32"),
        ("loadFloat", "rt.load.f64"),
        ("loadString", "rt.load.string"),
        ("allocString", "wasm.func_list.__heap_alloc_string"),
        ("allocVec", "wasm.func_list.__heap_alloc_vec"),
        ("functions", "wasm.table_list.__indirect_function_table"),
        ("invokeFunction", "util.invokeFunction"),
        ("dropFunctionRef", "util.dropFunctionRef"),
    ];

    Stream::expression(|stream| {
        let names = paths.iter().map(|v| v.0).collect::<Vec<_>>().join(", ");
        let values = paths.iter().map(|v| v.1).collect::<Vec<_>>().join(", ");

        note!(stream, "local abi = {{ ffi = {{}} }};");
        note!(stream, "local {names};");
        note!(stream, "local connections = {{}};");

        note!(stream, "{LUAU_CREATE_CONNECTION}");

        push!(stream, "function abi.load(wasm, rt, util)");
        note!(stream, "{names} = {values};");
        pull!(stream, "end");

        note!(stream, "{LUAU_DISCONNECT_CONNECTION}");
    })
}

fn generate_lua_value_conversion() -> String {
    Stream::expression(|stream| {
        push!(stream, "local function lua_value_number(value)");
        note!(stream, "return createPointer(value);");
        pull!(stream, "end");

        for number in LUA_VALUE_NUMBER_TYPES {
            note!(stream, "abi.ffi.lua_value_{number} = lua_value_number;");
        }
    })
}

fn generate_abi_tail() -> String {
    "return abi;".to_string()
}

pub fn generate(namespaces: &[Namespace]) -> String {
    let mut output = Vec::new();
    output.push(generate_abi_start());

    for namespace in namespaces {
        generate_namespace(&mut output, namespace);
    }

    output.push(generate_lua_value_conversion());
    output.push(generate_abi_tail());
    output.join("\n")
}
