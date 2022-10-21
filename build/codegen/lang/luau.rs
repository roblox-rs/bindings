use crate::codegen::{
    constants::{
        luau::{
            LUAU_CREATE_CONNECTION, LUAU_DISCONNECT_CONNECTION, LUAU_LUA_VALUE_CONVERSION,
            LUAU_LUA_VALUE_CONVERSION_VARIABLE, LUA_VALUE_NUMBER_TYPES,
        },
        MULTI_VALUE_SUPPORT,
    },
    lang::callback_extern_name,
    stream::{note, pull, push, Stream},
    structs::{
        Class, ClassCallbackMember, ClassEventMember, ClassFunctionMember, ClassFunctionParameter,
        ClassMember, ClassPropertyMember, Dump, GroupType, PrimitiveKind, ValueType,
    },
};

use super::{dyn_fn_extern_name, event_extern_name, get_prop_extern_name, set_prop_extern_name};

fn get_ffi_size(value_type: &ValueType) -> i32 {
    match value_type {
        ValueType::Primitive(PrimitiveKind::String) => 2,
        ValueType::Class(_)
        | ValueType::DataType(_)
        | ValueType::Primitive(_)
        | ValueType::Enum(_) => 1,
        ValueType::Optional(value_type) => 1 + get_ffi_size(value_type),
        ValueType::DetailedGroup(group) => match group.as_ref() {
            GroupType::Array(_) | GroupType::Tuple(_) => 2,
            GroupType::Variant => 1,
            _ => panic!("Unhandled group type"),
        },
        _ => panic!("Unhandled value type"),
    }
}

fn format_parameter_name(parameter: &ClassFunctionParameter) -> String {
    if parameter.value_type.is_multi_value() {
        "...".to_string()
    } else {
        format!("p_{}", &parameter.name)
    }
}

fn new_variable_id(variable_id: &mut i32) -> i32 {
    let id = *variable_id;
    *variable_id += 1;
    id
}

fn convert_ffi_input_to_lua_value(
    writer: &mut Stream,
    value_type: &ValueType,
    parameters: &[String],
    variable_id: &mut i32,
) -> String {
    match value_type {
        ValueType::Enum(_) => parameters[0].clone(),
        ValueType::Class(_) | ValueType::DataType(_) => format!("getPointer({})", parameters[0]),
        ValueType::Primitive(PrimitiveKind::String) => {
            format!("loadString(memory, {}, {})", parameters[0], parameters[1])
        }
        ValueType::Primitive(PrimitiveKind::Bool) => format!("{} == 1", parameters[0]),
        ValueType::Primitive(_) => parameters[0].to_string(),
        ValueType::Optional(value_type) => {
            let id = new_variable_id(variable_id);
            note!(writer, "local value{id};");
            push!(writer, "if {} == 1 then", parameters[0]);
            let result =
                convert_ffi_input_to_lua_value(writer, value_type, &parameters[1..], variable_id);
            note!(writer, "value{id} = {};", result);
            pull!(writer, "end");
            format!("value{id}")
        }
        ValueType::DetailedGroup(group) => match group.as_ref() {
            GroupType::Array(value_type) | GroupType::Tuple(value_type) => {
                let id = new_variable_id(variable_id);
                let size = get_ffi_size(value_type);
                note!(writer, "local result{id} = table.create({})", parameters[1]);
                push!(writer, "for i = 1, {} do", parameters[1]);
                for i in 0..size {
                    note!(
                        writer,
                        "local value{id}_{i} = loadU32(memory, {} + (i - 1) * {} + {});",
                        parameters[0],
                        size * 4,
                        i * 4
                    );
                }
                let result = convert_ffi_input_to_lua_value(
                    writer,
                    value_type,
                    &(0..size)
                        .map(|v| format!("value{id}_{v}"))
                        .collect::<Vec<_>>()[..],
                    variable_id,
                );
                note!(writer, "result{id}[i] = {}", result);
                pull!(writer, "end");
                if matches!(group.as_ref(), GroupType::Tuple(_)) {
                    format!("unpack(result{id}, 1, {})", parameters[1])
                } else {
                    format!("result{id}")
                }
            }
            GroupType::Variant => format!("getPointer({})", parameters[0]),
            _ => panic!("Unhandled group type"),
        },
        _ => "nil --[[ unimplemented ]]".to_string(),
    }
}

fn convert_lua_value_to_ffi_output(
    writer: &mut Stream,
    value_type: &ValueType,
    value: &str,
    variable_id: &mut i32,
) -> Vec<String> {
    match value_type {
        ValueType::Enum(_) => vec![format!("({value}).Value")],
        ValueType::Class(_) | ValueType::DataType(_) => vec![format!("createPointer({value})")],
        ValueType::Primitive(PrimitiveKind::String) => {
            let id = new_variable_id(variable_id);
            let variable = format!("value{id}");
            note!(writer, "local {variable} = {value};");
            note!(writer, "local stringContent{id}, stringLength{id} = allocString(#{variable}), #{variable};");
            push!(writer, "for i = 1, stringLength{id} do");
            note!(
                writer,
                "storeU8(memory, stringContent{id} + (i - 1), string.byte({variable}:sub(i, i)));"
            );
            pull!(writer, "end");
            vec![format!("stringContent{id}"), format!("stringLength{id}")]
        }
        ValueType::Primitive(PrimitiveKind::Bool) => vec![format!("{value} and 1 or 0")],
        ValueType::Primitive(_) => vec![value.to_string()],
        ValueType::Optional(value_type) => {
            let id = new_variable_id(variable_id);
            let variable = format!("value{id}");
            note!(writer, "local {variable} = {};", value);

            let values =
                convert_lua_value_to_ffi_output(writer, value_type, &variable, variable_id);
            if values.len() == 1 {
                vec![
                    format!("{variable} and 1 or 0"),
                    format!("{variable} and {} or 0", values[0]),
                ]
            } else {
                unimplemented!("Optional fields require a single usize value");
            }
        }
        ValueType::DetailedGroup(group) => match group.as_ref() {
            GroupType::Array(value_type) => {
                let id = new_variable_id(variable_id);
                note!(writer, "local result{id} = {value};");
                note!(
                    writer,
                    "local vec{id} = allocVec(#result{id} * {});",
                    get_ffi_size(value_type) * 4
                );
                push!(writer, "for i, v in ipairs(result{id}) do");
                let result = convert_lua_value_to_ffi_output(writer, value_type, "v", variable_id);
                for (i, expression) in result.iter().enumerate() {
                    note!(
                        writer,
                        "storeU32(memory, vec{id} + (i - 1) * {} + {}, {expression})",
                        result.len() * 4,
                        i * 4
                    );
                }
                pull!(writer, "end");
                vec![format!("vec{id}"), format!("#result{id}")]
            }
            GroupType::Tuple(value_type) => {
                let id = new_variable_id(variable_id);
                let is_dots = value == "...";
                if is_dots {
                    note!(writer, "local length{id} = select('#', ...);");
                } else {
                    note!(writer, "local value{id} = {value};");
                    note!(writer, "local length{id} = #value{id};");
                }
                note!(
                    writer,
                    "local vec{id} = allocVec(length{id} * {});",
                    get_ffi_size(value_type)
                );
                push!(writer, "for i = 1, length{id} do");
                let result = convert_lua_value_to_ffi_output(
                    writer,
                    value_type,
                    &if is_dots {
                        "select(i, ...)".to_string()
                    } else {
                        format!("value{id}[i]")
                    },
                    variable_id,
                );
                for (i, v) in result.iter().enumerate() {
                    note!(
                        writer,
                        "storeU32(memory, vec{id} + (i - 1) * {} + {}, {v})",
                        result.len() * 4,
                        i
                    );
                }
                pull!(writer, "end");
                vec![format!("vec{id}"), format!("length{id}")]
            }
            GroupType::Variant => vec![format!("createPointer({value})")],
            _ => unimplemented!(),
        },
        _ => panic!("Unhandled value type"),
    }
}

fn store_in_memory_or_return(writer: &mut Stream, result: Vec<String>) {
    if result.len() == 1 || MULTI_VALUE_SUPPORT {
        note!(writer, "return {};", result.join(", "));
    } else {
        for (i, v) in result.iter().enumerate() {
            note!(writer, "storeU32(memory, output + {}, {v})", i * 4);
        }
    }
}

fn generate_class_property(writer: &mut Stream, class: &Class, member: &ClassPropertyMember) {
    push!(
        writer,
        "function abi.ffi.{}({})",
        get_prop_extern_name(class, member),
        if get_ffi_size(&member.value_type) > 1 && !MULTI_VALUE_SUPPORT {
            "output, instance"
        } else {
            "instance"
        },
    );
    let result = convert_lua_value_to_ffi_output(
        writer,
        &member.value_type,
        &format!("getPointer(instance).{}", member.name),
        &mut 0,
    );
    store_in_memory_or_return(writer, result);
    pull!(writer, "end");

    if !member.tags.contains("ReadOnly") {
        let mut input_parameters = Vec::<String>::new();
        for i in 0..get_ffi_size(&member.value_type) {
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
            input_parameters.as_slice(),
            &mut 0,
        );
        note!(writer, "getPointer(instance).{} = {};", member.name, result);
        pull!(writer, "end");
    }
}

fn generate_class_function(writer: &mut Stream, class: &Class, member: &ClassFunctionMember) {
    let mut input_parameters = vec!["instance".to_string()];
    if get_ffi_size(&member.return_type) > 1 && !MULTI_VALUE_SUPPORT {
        input_parameters.insert(0, "output".to_string());
    }

    for parameter in &member.parameters {
        for i in 0..get_ffi_size(&parameter.value_type) {
            input_parameters.push(format!("p_{}{i}", &parameter.name));
        }
    }

    push!(
        writer,
        "function abi.ffi.{}({})",
        dyn_fn_extern_name(class, member),
        input_parameters.join(", ")
    );

    let mut output_parameters = Vec::<String>::new();
    let mut variable_id = 0;
    for parameter in &member.parameters {
        let mut parameters = vec![];
        for i in 0..get_ffi_size(&parameter.value_type) {
            parameters.push(format!("p_{}{i}", &parameter.name));
        }

        output_parameters.push(convert_ffi_input_to_lua_value(
            writer,
            &parameter.value_type,
            parameters.as_slice(),
            &mut variable_id,
        ));
    }

    let value = if member.return_type.is_multi_value() {
        format!(
            "{{ getPointer(instance):{}({}) }}",
            member.name,
            output_parameters.join(", ")
        )
    } else {
        format!(
            "getPointer(instance):{}({})",
            member.name,
            output_parameters.join(", ")
        )
    };
    let result = convert_lua_value_to_ffi_output(writer, &member.return_type, &value, &mut 0);
    store_in_memory_or_return(writer, result);
    pull!(writer, "end");
}

fn generate_class_event(writer: &mut Stream, class: &Class, member: &ClassEventMember) {
    let mut input_parameters = Vec::<String>::new();
    for parameter in &member.parameters {
        input_parameters.push(format_parameter_name(parameter));
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

    let mut variable_id = 0;
    for parameter in &member.parameters {
        parameters.append(&mut convert_lua_value_to_ffi_output(
            writer,
            &parameter.value_type,
            &format_parameter_name(parameter),
            &mut variable_id,
        ));
    }

    note!(writer, "invokeFunction({});", parameters.join(", "));
    pull!(writer, "end))");
    pull!(writer, "end");
}

fn generate_class_callback(writer: &mut Stream, class: &Class, member: &ClassCallbackMember) {
    let mut input_parameters = Vec::<String>::new();
    for parameter in &member.parameters {
        input_parameters.push(format_parameter_name(parameter));
    }

    push!(
        writer,
        "function abi.ffi.{}(instance, destructor, data, vtable)",
        callback_extern_name(class, member)
    );
    push!(
        writer,
        "getPointer(instance).{} = function({})",
        member.name,
        input_parameters.join(", ")
    );

    let mut parameters = Vec::<String>::new();
    parameters.push("data".to_string());
    parameters.push("vtable".to_string());

    let mut variable_id = 0;
    for parameter in &member.parameters {
        parameters.append(&mut convert_lua_value_to_ffi_output(
            writer,
            &parameter.value_type,
            &format_parameter_name(parameter),
            &mut variable_id,
        ));
    }

    let mut results = Vec::<String>::with_capacity(get_ffi_size(&member.return_type) as usize);
    for _ in 0..get_ffi_size(&member.return_type) {
        results.push(format!("response{}", new_variable_id(&mut variable_id)));
    }

    note!(
        writer,
        "local {} = invokeFunction({})",
        results.join(", "),
        parameters.join(", ")
    );

    let response =
        convert_ffi_input_to_lua_value(writer, &member.return_type, &results[..], &mut variable_id);

    note!(
        writer,
        "functions.data[destructor]({});",
        results.join(", ")
    );

    note!(writer, "return {response};",);
    pull!(writer, "end");
    pull!(writer, "end");
}

fn generate_class(writer: &mut Stream, class: &Class) {
    for member in &class.members {
        match member {
            ClassMember::Property(property) => generate_class_property(writer, class, property),
            ClassMember::Function(function) => generate_class_function(writer, class, function),
            ClassMember::Callback(callback) => generate_class_callback(writer, class, callback),
            ClassMember::Event(event) => generate_class_event(writer, class, event),
        }
    }
}

fn generate_classes(writer: &mut Stream, dump: &Dump) {
    for class in &dump.classes {
        generate_class(writer, class);
    }
}

fn generate_lua_value_conversion(writer: &mut Stream) {
    push!(writer, "function lua_value_number(value)");
    note!(writer, "return createPointer(value);");
    pull!(writer, "end");

    note!(writer, "{}", LUAU_LUA_VALUE_CONVERSION);
    note!(writer, "{}", LUAU_LUA_VALUE_CONVERSION_VARIABLE);

    for number in LUA_VALUE_NUMBER_TYPES {
        note!(writer, "abi.ffi.lua_value_{number} = lua_value_number;");
    }
}

fn generate_abi_start(writer: &mut Stream) {
    let paths = vec![
        ("getPointer", "util.getPointer"),
        ("createPointer", "util.createPointer"),
        ("memory", "wasm.memory_list.memory"),
        ("storeU8", "rt.store.i32_n8"),
        ("storeU32", "rt.store.i32"),
        ("loadU8", "rt.load.i32_u8"),
        ("loadU32", "rt.load.i32"),
        ("loadString", "rt.load.string"),
        ("allocString", "wasm.func_list.__heap_alloc_string"),
        ("allocVec", "wasm.func_list.__heap_alloc_vec"),
        ("functions", "wasm.table_list.__indirect_function_table"),
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

    note!(writer, "{}", LUAU_CREATE_CONNECTION);

    push!(writer, "function abi.load(wasm, rt, util)");
    note!(writer, "{names} = {values};");
    pull!(writer, "end");

    note!(writer, "{}", LUAU_DISCONNECT_CONNECTION);
}

fn generate_abi_tail(writer: &mut Stream) {
    note!(writer, "return abi;");
}

pub fn generate(dump: &Dump) -> String {
    let mut writer = Stream::new();
    note!(writer, "-- Generated by roblox-rs");
    generate_abi_start(&mut writer);
    generate_classes(&mut writer, dump);
    generate_lua_value_conversion(&mut writer);
    generate_abi_tail(&mut writer);
    writer.stream
}
