use std::{collections::HashMap, iter::FromIterator};

use crate::codegen::{
    stream::{note, pull, push, Stream},
    structs::{
        Class, ClassEventMember, ClassFunctionMember, ClassMember, ClassPropertyMember, Dump,
        PrimitiveKind, ValueType,
    },
};

use super::{
    dyn_fn_extern_name, event_extern_name, get_prop_extern_name, set_prop_extern_name, to_pascal,
};

fn get_ffi_input_count(value_type: &ValueType) -> i32 {
    match value_type {
        ValueType::Primitive(PrimitiveKind::String) => 2,
        ValueType::Class(_) | ValueType::DataType(_) | ValueType::Primitive(_) => 1,
        ValueType::Optional(value_type) => 1 + get_ffi_input_count(value_type),
        _ => panic!("Unhandled value type"),
    }
}

fn convert_ffi_input_to_lua_value(
    writer: &mut Stream,
    value_type: &ValueType,
    value: &str,
    parameters: &[String],
    raw_suffix: &str,
    increment: i32,
) -> String {
    let suffix = format!("{raw_suffix}{increment}");
    match value_type {
        ValueType::Class(_) | ValueType::DataType(_) => format!("getPointer({})", parameters[0]),
        ValueType::Primitive(PrimitiveKind::String) => {
            note!(writer, "local string{suffix} = \"\";");
            push!(writer, "for i = 1, {} do", parameters[1]);
            note!(
                writer,
                "string{suffix} ..= string.char(loadU8(memory, {} + (i - 1)))",
                parameters[0]
            );
            pull!(writer, "end");
            format!("string{suffix}")
        }
        ValueType::Primitive(PrimitiveKind::Bool) => format!("{} == 1", parameters[0]),
        ValueType::Primitive(_) => parameters[0].to_string(),
        ValueType::Optional(value_type) => {
            note!(writer, "local value{suffix};");
            push!(writer, "if {} == 1 then", parameters[0]);
            let result = convert_ffi_input_to_lua_value(
                writer,
                value_type,
                value,
                &parameters[1..],
                raw_suffix,
                increment + 1,
            );
            note!(writer, "value{suffix} = {};", result);
            pull!(writer, "end");
            format!("value{suffix}")
        }
        _ => format!("{value} --[[ unimplemented ]]"),
    }
}

fn convert_lua_value_to_ffi_output(
    writer: &mut Stream,
    value_type: &ValueType,
    value: &str,
    raw_suffix: &str,
    increment: i32,
) -> String {
    let suffix = format!("{raw_suffix}{increment}");
    match value_type {
        ValueType::Class(_) | ValueType::DataType(_) => format!("createPointer({value})"),
        ValueType::Primitive(PrimitiveKind::String) => {
            let variable = format!("value{suffix}");
            note!(writer, "local {variable} = {value};");
            note!(writer, "local stringContent{suffix}, stringLength{suffix} = allocString(#{variable}), #{variable};");
            push!(writer, "for i = 1, stringLength{suffix} do");
            note!(
                writer,
                "storeU8(memory, stringContent{suffix} + (i - 1), string.byte({variable}:sub(i, i)));"
            );
            pull!(writer, "end");
            format!("stringContent{suffix}, stringLength{suffix}")
        }
        ValueType::Primitive(PrimitiveKind::Bool) => format!("{value} and 1 or 0"),
        ValueType::Primitive(_) => value.to_string(),
        ValueType::Optional(value_type) => {
            let variable = format!("value{suffix}");
            note!(writer, "local {variable} = {};", value);
            format!(
                "{variable} and 1 or 0, {variable} and {} or 0",
                convert_lua_value_to_ffi_output(
                    writer, value_type, &variable, raw_suffix, increment
                )
            )
        }
        _ => panic!("Unhandled value type"),
    }
}

fn generate_class_property(writer: &mut Stream, class: &Class, member: &ClassPropertyMember) {
    push!(
        writer,
        "function abi.ffi.{}(instance)",
        get_prop_extern_name(class, member)
    );
    let result = convert_lua_value_to_ffi_output(
        writer,
        &member.value_type,
        &format!("getPointer(instance).{}", member.name),
        "Prop",
        0,
    );
    note!(writer, "return {result};");
    pull!(writer, "end");

    if !member.tags.contains("ReadOnly") {
        let mut input_parameters = Vec::<String>::new();
        for i in 0..get_ffi_input_count(&member.value_type) {
            input_parameters.push(format!("p_{}{i}", member.name));
        }

        push!(
            writer,
            "function abi.ffi.{}(instance, {})",
            set_prop_extern_name(class, member),
            input_parameters.join(", ")
        );
        let result = convert_ffi_input_to_lua_value(
            writer,
            &member.value_type,
            &format!("p_{}", member.name),
            input_parameters.as_slice(),
            "value",
            0,
        );
        note!(writer, "getPointer(instance).{} = {};", member.name, result);
        pull!(writer, "end");
    }
}

fn generate_class_function(writer: &mut Stream, class: &Class, member: &ClassFunctionMember) {
    let mut input_parameters = vec!["instance".to_string()];
    for parameter in &member.parameters {
        for i in 0..get_ffi_input_count(&parameter.value_type) {
            input_parameters.push(format!("p_{}{i}", parameter.name));
        }
    }

    push!(
        writer,
        "function abi.ffi.{}({})",
        dyn_fn_extern_name(class, member),
        input_parameters.join(", ")
    );

    let mut output_parameters = Vec::<String>::new();
    for parameter in &member.parameters {
        let mut parameters = vec![];
        for i in 0..get_ffi_input_count(&parameter.value_type) {
            parameters.push(format!("p_{}{i}", parameter.name));
        }

        output_parameters.push(convert_ffi_input_to_lua_value(
            writer,
            &parameter.value_type,
            &format!("p_{}", parameter.name),
            parameters.as_slice(),
            &format!("p{}", to_pascal(&parameter.name)),
            0,
        ));
    }

    let result = convert_lua_value_to_ffi_output(
        writer,
        &member.return_type,
        &format!(
            "getPointer(instance):{}({})",
            member.name,
            output_parameters.join(", ")
        ),
        "Return",
        0,
    );
    note!(writer, "return {result};");
    pull!(writer, "end");
}

fn generate_class_event(writer: &mut Stream, class: &Class, member: &ClassEventMember) {
    let mut input_parameters = Vec::<String>::new();
    for parameter in &member.parameters {
        input_parameters.push(format!("p_{}", parameter.name));
    }

    push!(
        writer,
        "function abi.ffi.{}(instance, data, vtable)",
        event_extern_name(class, member)
    );
    push!(
        writer,
        "return createConnection(data, vtable, getPointer(instance).{}:Connect(function({})",
        member.event_name.as_ref().unwrap_or(&member.name),
        input_parameters.join(", ")
    );

    let mut parameters = Vec::<String>::new();
    parameters.push("data".to_string());
    parameters.push("vtable".to_string());

    for parameter in &member.parameters {
        parameters.push(convert_lua_value_to_ffi_output(
            writer,
            &parameter.value_type,
            &format!("p_{}", parameter.name),
            "",
            0,
        ));
    }

    note!(writer, "invokeFunction({});", parameters.join(", "));
    pull!(writer, "end))");
    pull!(writer, "end");
}

fn generate_class(writer: &mut Stream, class: &Class) {
    for member in &class.members {
        match member {
            ClassMember::Property(property) => generate_class_property(writer, class, property),
            ClassMember::Function(function) => generate_class_function(writer, class, function),
            ClassMember::Event(event) => generate_class_event(writer, class, event),
            _ => {}
        }
    }
}

fn generate_classes(writer: &mut Stream, dump: &Dump) {
    for class in &dump.classes {
        generate_class(writer, class);
    }
}

fn generate_abi_start(writer: &mut Stream) {
    let paths = vec![
        ("getPointer", "util.getPointer"),
        ("createPointer", "util.createPointer"),
        ("memory", "wasm.memory_list.memory"),
        ("storeU8", "rt.store.i32_n8"),
        ("loadU8", "rt.load.i32_u8"),
        ("allocString", "wasm.func_list.__heap_alloc_string"),
        ("invokeFunction", "util.invokeFunction"),
        ("dropFunctionRef", "util.dropFunctionRef"),
    ];

    let names = paths
        .clone()
        .into_iter()
        .map(|v| v.0)
        .collect::<Vec<&'static str>>()
        .join(", ");

    let values = paths
        .clone()
        .into_iter()
        .map(|v| v.1)
        .collect::<Vec<&'static str>>()
        .join(", ");

    note!(writer, "local abi = {{ ffi = {{}} }};");
    note!(writer, "local {names};");
    note!(writer, "local connections = {{}};");

    push!(
        writer,
        "local function createConnection(stack, vtable, connection)"
    );
    note!(writer, "local id = createPointer(connection);");
    note!(writer, "connections[id] = {{ stack, vtable }};");
    note!(writer, "return id;");
    pull!(writer, "end");

    push!(writer, "function abi.load(wasm, rt, util)");
    note!(writer, "{names} = {values};");
    pull!(writer, "end");

    push!(writer, "function abi.ffi.disconnect_connection(connection)");
    note!(writer, "local func = connections[connection];");
    note!(writer, "if not func then return end");
    note!(writer);
    note!(writer, "getPointer(connection):Disconnect();");
    note!(writer, "dropFunctionRef(func[1], func[2]);");
    note!(writer, "connections[connection] = nil;");
    pull!(writer, "end");
}

fn generate_abi_tail(writer: &mut Stream) {
    note!(writer, "return abi;");
}

pub fn generate(dump: &Dump) -> String {
    let mut writer = Stream::new();
    note!(writer, "-- Generated by roblox-rs");
    generate_abi_start(&mut writer);
    generate_classes(&mut writer, dump);
    generate_abi_tail(&mut writer);
    writer.stream
}
