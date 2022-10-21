pub mod constants;
pub mod lang;
pub mod stream;
pub mod structs;

use structs::{
    Class, ClassEventMember, ClassFunctionMember, ClassFunctionParameter, ClassMember,
    ClassPropertyMember, DataTypeKind, Dump, PrimitiveKind, Security, ValueType,
};

use crate::{
    config::{COMPLEX_GROUP_TYPES, NON_OPTIONAL_EVENT_PARAMETERS, OPTIONAL_FUNCTION_PARAMETERS},
    CLASS_BLACKLIST,
};

use self::{
    lang::{is_camel_case, to_snake},
    structs::{ClassCallbackMember, GroupType},
};

enum SecurityKind {
    Read,
    Write,
    Either,
}

enum GroupSource {
    Parameter(usize),
    Result,
}

fn get_fallback_group(value_type: &ValueType) -> Option<GroupType> {
    if let ValueType::Group(name) = value_type {
        if name == "Variant" || name == "Dictionary" {
            return Some(GroupType::Variant);
        } else {
            return None;
        }
    }

    None
}

fn get_detailed_group(
    class: &Class,
    member: &ClassMember,
    kind: &GroupSource,
    value_type: &ValueType,
) -> Option<GroupType> {
    let config_name = format!(
        "{}.{}({})",
        &class.name,
        &member.name(),
        match kind {
            GroupSource::Parameter(idx) => idx.to_string(),
            _ => "".to_string(),
        }
    );

    let detailed_group = COMPLEX_GROUP_TYPES.get(config_name.as_str());
    detailed_group
        .cloned()
        .or_else(|| get_fallback_group(value_type))
}

fn should_generate_class(dump: &Dump, class: &Class) -> bool {
    if CLASS_BLACKLIST.contains(class.name.as_str()) {
        false
    } else {
        dump.parent(class)
            .map_or(true, |superclass| should_generate_class(dump, superclass))
    }
}

fn is_type_generated(dump: &Dump, value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Class(kind) => should_generate_class(dump, dump.class(kind)),
        ValueType::DataType(
            DataTypeKind::BinaryString
            | DataTypeKind::Function
            | DataTypeKind::QDir
            | DataTypeKind::QFont
            | DataTypeKind::ProtectedString
            | DataTypeKind::Objects
            | DataTypeKind::RbxScriptSignal,
        )
        | ValueType::Group(_) => false,
        _ => true,
    }
}

fn is_group_type_generated(
    class: &Class,
    member: &ClassMember,
    kind: &GroupSource,
    value_type: &ValueType,
) -> bool {
    if let ValueType::Group(_) | ValueType::DataType(DataTypeKind::Objects) = value_type {
        get_detailed_group(class, member, kind, value_type).is_some()
    } else {
        false
    }
}

fn is_security_accessible(security: &Security, kind: &SecurityKind) -> bool {
    let security_level = match security {
        Security::Uniform(value) => value,
        Security::NonUniform(non_uniform) => match kind {
            SecurityKind::Read => &non_uniform.read,
            SecurityKind::Write => &non_uniform.write,
            SecurityKind::Either => {
                return is_security_accessible(security, &SecurityKind::Read)
                    || is_security_accessible(security, &SecurityKind::Write)
            }
        },
    };

    security_level == "None"
}

fn transform_value_type(value_type: &ValueType) -> ValueType {
    match value_type {
        ValueType::DataType(DataTypeKind::PossibleCFrame) => {
            ValueType::Optional(Box::new(ValueType::DataType(DataTypeKind::CFrame)))
        }
        ValueType::Primitive(PrimitiveKind::PossibleFloat) => {
            ValueType::Optional(Box::new(ValueType::Primitive(PrimitiveKind::Float)))
        }
        ValueType::Class(kind) => ValueType::Optional(Box::new(ValueType::Class(kind.clone()))),
        _ => value_type.clone(),
    }
}

fn transform_detailed_value_type(
    class: &Class,
    member: &ClassMember,
    kind: &GroupSource,
    value_type: &ValueType,
) -> ValueType {
    if let ValueType::Group(_) | ValueType::DataType(DataTypeKind::Objects) = value_type {
        ValueType::DetailedGroup(Box::new(
            get_detailed_group(class, member, kind, value_type).unwrap(),
        ))
    } else {
        transform_value_type(value_type)
    }
}

fn transform_parameter_value_type(
    class: &Class,
    member: &ClassMember,
    kind: &GroupSource,
    value_type: &ValueType,
) -> ValueType {
    // Function arguments should not have optional-by-default arguments.
    if let ValueType::Class(_) = value_type {
        return value_type.clone();
    }

    transform_detailed_value_type(class, member, kind, value_type)
}

fn is_valid_class_member(class: &Class, member: &ClassMember) -> bool {
    if !is_security_accessible(member.security(), &SecurityKind::Either) {
        return false;
    }

    if member.tags().contains("NotScriptable") {
        return false;
    }

    // Remove deprecated members at least temporarily
    if member.tags().contains("Deprecated") {
        return false;
    }

    // CommandInstance redefines Name
    if class.name == "CommandInstance" && member.name() == "Name" {
        return false;
    }

    // Remove Roblox's "pseudo properties"
    if member.name().contains(' ') {
        return false;
    }

    // Remove camelCase variants.
    if is_camel_case(member.name()) {
        return false;
    }

    true
}

fn transform_class_property(dump: &Dump, property: &ClassPropertyMember) -> Option<ClassMember> {
    if !is_type_generated(dump, &property.value_type) {
        return None;
    }

    let property = ClassPropertyMember {
        value_type: transform_value_type(&property.value_type),
        ..property.clone()
    };

    Some(ClassMember::Property(property))
}

fn transform_class_function(
    dump: &Dump,
    class: &Class,
    function: &ClassFunctionMember,
) -> Option<ClassMember> {
    if !is_type_generated(dump, &function.return_type)
        && !is_group_type_generated(
            class,
            &function.clone().into(),
            &GroupSource::Result,
            &function.return_type,
        )
    {
        return None;
    }

    // RbxScriptSignal isn't useful, so we should not generate these functions.
    if let ValueType::DataType(DataTypeKind::RbxScriptSignal) = &function.return_type {
        return None;
    }

    if !function
        .parameters
        .iter()
        .enumerate()
        .all(|(i, parameter)| {
            is_type_generated(dump, &parameter.value_type)
                || is_group_type_generated(
                    class,
                    &function.clone().into(),
                    &GroupSource::Parameter(i),
                    &parameter.value_type,
                )
        })
    {
        return None;
    }

    let mut function = ClassFunctionMember {
        return_type: transform_detailed_value_type(
            class,
            &function.clone().into(),
            &GroupSource::Result,
            &function.return_type,
        ),
        parameters: function
            .parameters
            .iter()
            .enumerate()
            .map(|(i, v)| ClassFunctionParameter {
                value_type: transform_parameter_value_type(
                    class,
                    &function.clone().into(),
                    &GroupSource::Parameter(i),
                    &v.value_type,
                ),
                ..v.clone()
            })
            .collect(),
        ..function.clone()
    };

    let function_name = to_snake(&function.name);

    // For cases like SetAxis() and Axis =
    if class
        .members
        .iter()
        .filter(|value| value.name() != function.name && !is_camel_case(value.name()))
        .any(|value| {
            let member_name = to_snake(value.name());
            function_name == member_name || format!("set_{member_name}") == function_name
        })
    {
        function.func_name = Some(function.name.clone());
        function.name = format!("Fn{}", function.name);
    }

    let qualified_name = format!("{}.{}", &class.name, &function.name);
    if let Some(optional) = OPTIONAL_FUNCTION_PARAMETERS.get(qualified_name.as_str()) {
        for (i, v) in function.parameters.iter_mut().enumerate() {
            if optional.contains(&i) {
                v.value_type = ValueType::Optional(Box::new(v.value_type.clone()));
            }
        }
    }

    // Clone is shadowed by impl Clone and Move is a keyword.
    if function.name == "Clone" || function.name == "Move" {
        function.func_name = Some(function.name.clone());
        function.name = format!("Fn{}", &function.name);
    }

    Some(ClassMember::Function(function))
}

fn transform_class_event(
    dump: &Dump,
    class: &Class,
    event: &ClassEventMember,
) -> Option<ClassMember> {
    if !event.parameters.iter().enumerate().all(|(i, parameter)| {
        is_type_generated(dump, &parameter.value_type)
            || is_group_type_generated(
                class,
                &event.clone().into(),
                &GroupSource::Parameter(i),
                &parameter.value_type,
            )
    }) {
        return None;
    }

    let mut event = ClassEventMember {
        parameters: event
            .parameters
            .iter()
            .enumerate()
            .map(|(i, v)| ClassFunctionParameter {
                value_type: transform_detailed_value_type(
                    class,
                    &event.clone().into(),
                    &GroupSource::Parameter(i),
                    &v.value_type,
                ),
                ..v.clone()
            })
            .collect(),
        ..event.clone()
    };

    if class.name == "Instance" && event.name == "Changed" {
        event.name = "InstanceChanged".into();
        event.event_name = Some("Changed".into());
    }

    if event.name.starts_with("On") {
        event.event_name = Some(event.name.clone());
        event.name = event.name[2..].to_string();
    }

    let qualified_name = format!(
        "{}.{}",
        &class.name,
        &event.event_name.as_ref().unwrap_or(&event.name)
    );
    if let Some(non_optional) = NON_OPTIONAL_EVENT_PARAMETERS.get(qualified_name.as_str()) {
        for (i, v) in event.parameters.iter_mut().enumerate() {
            if non_optional.contains(&i) {
                if let ValueType::Optional(value_type) = &v.value_type {
                    v.value_type = *value_type.clone();
                }
            }
        }
    }

    Some(ClassMember::Event(event))
}

fn transform_class_callback(
    dump: &Dump,
    class: &Class,
    callback: &ClassCallbackMember,
) -> Option<ClassMember> {
    if !callback
        .parameters
        .iter()
        .enumerate()
        .all(|(i, parameter)| {
            is_type_generated(dump, &parameter.value_type)
                || is_group_type_generated(
                    class,
                    &callback.clone().into(),
                    &GroupSource::Parameter(i),
                    &parameter.value_type,
                )
        })
    {
        return None;
    }

    if !is_type_generated(dump, &callback.return_type)
        && !is_group_type_generated(
            class,
            &callback.clone().into(),
            &GroupSource::Result,
            &callback.return_type,
        )
    {
        return None;
    }

    let mut callback = ClassCallbackMember {
        return_type: transform_detailed_value_type(
            class,
            &callback.clone().into(),
            &GroupSource::Result,
            &callback.return_type,
        ),
        parameters: callback
            .parameters
            .iter()
            .enumerate()
            .map(|(i, v)| ClassFunctionParameter {
                value_type: transform_detailed_value_type(
                    class,
                    &callback.clone().into(),
                    &GroupSource::Parameter(i),
                    &v.value_type,
                ),
                ..v.clone()
            })
            .collect(),
        ..callback.clone()
    };

    let qualified_name = format!("{}.{}", &class.name, &callback.name);
    if let Some(non_optional) = NON_OPTIONAL_EVENT_PARAMETERS.get(qualified_name.as_str()) {
        for (i, v) in callback.parameters.iter_mut().enumerate() {
            if non_optional.contains(&i) {
                if let ValueType::Optional(value_type) = &v.value_type {
                    v.value_type = *value_type.clone();
                }
            }
        }
    }

    Some(ClassMember::Callback(callback))
}

fn transform_class_member(dump: &Dump, class: &Class, member: &ClassMember) -> Option<ClassMember> {
    if !is_valid_class_member(class, member) {
        return None;
    }

    match member {
        ClassMember::Property(property) => transform_class_property(dump, property),
        ClassMember::Function(function) => transform_class_function(dump, class, function),
        ClassMember::Event(event) => transform_class_event(dump, class, event),
        ClassMember::Callback(callback) => transform_class_callback(dump, class, callback),
    }
}

fn transform_class(dump: &Dump, class: Class) -> Option<Class> {
    if should_generate_class(dump, &class) {
        Some(Class {
            members: class
                .members
                .iter()
                .filter_map(|member| {
                    let result = transform_class_member(dump, &class, member);
                    if is_valid_class_member(&class, member) && result.is_none() {
                        println!("cant generate: {}.{}", class.name, member.name());
                    };
                    result
                })
                .collect(),
            name: class.name,
            tags: class.tags,
            superclass: class.superclass,
        })
    } else {
        None
    }
}

pub fn transform_dump(dump: &Dump) -> Dump {
    let mut new_dump = Dump {
        classes: Vec::new(),
        enums: dump.enums.clone(),
    };

    for class in dump.classes.iter().cloned() {
        if let Some(class) = transform_class(dump, class) {
            new_dump.classes.push(class);
        }
    }

    new_dump
}
