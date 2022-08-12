pub mod lang;
pub mod stream;
pub mod structs;
use structs::{
    Class, ClassEventMember, ClassFunctionMember, ClassFunctionParameter, ClassMember,
    ClassPropertyMember, DataTypeKind, Dump, PrimitiveKind, Security, ValueType,
};

use crate::{config::NON_OPTIONAL_EVENT_PARAMETERS, CLASS_BLACKLIST};

use self::lang::{is_camel_case, to_snake};

enum SecurityKind {
    Read,
    Write,
    Either,
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
        ValueType::Class(kind) => {
            return should_generate_class(dump, dump.class(kind));
        }
        ValueType::DataType(
            DataTypeKind::BinaryString
            | DataTypeKind::Function
            | DataTypeKind::QDir
            | DataTypeKind::QFont
            | DataTypeKind::ProtectedString
            | DataTypeKind::Objects,
        )
        | ValueType::Enum(_)
        | ValueType::Group(_) => {
            return false;
        }
        _ => {}
    }
    true
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
    if !is_type_generated(dump, &function.return_type) {
        return None;
    }

    // RbxScriptSignal isn't useful, so we should not generate these functions.
    if let ValueType::DataType(DataTypeKind::RbxScriptSignal) = &function.return_type {
        return None;
    }

    if !function
        .parameters
        .iter()
        .all(|parameter| is_type_generated(dump, &parameter.value_type))
    {
        return None;
    }

    let mut function = ClassFunctionMember {
        return_type: transform_value_type(&function.return_type),
        parameters: function
            .parameters
            .iter()
            .map(|v| ClassFunctionParameter {
                value_type: transform_value_type(&v.value_type),
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
    if !event
        .parameters
        .iter()
        .all(|parameter| is_type_generated(dump, &parameter.value_type))
    {
        return None;
    }

    let mut event = ClassEventMember {
        parameters: event
            .parameters
            .iter()
            .map(|v| ClassFunctionParameter {
                value_type: transform_value_type(&v.value_type),
                ..v.clone()
            })
            .collect(),
        ..event.clone()
    };

    if class.name == "Instance" && event.name == "Changed" {
        event.name = "InstanceChanged".into();
        event.event_name = Some("Changed".into());
    }

    let qualified_name = format!("{}.{}", &class.name, &event.name);
    if let Some(non_optional) = NON_OPTIONAL_EVENT_PARAMETERS.get(qualified_name.as_str()) {
        event.parameters = event
            .parameters
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if non_optional.contains(&i) {
                    ClassFunctionParameter {
                        name: v.name,
                        value_type: match v.value_type {
                            ValueType::Optional(value_type) => *value_type,
                            value_type => value_type,
                        },
                    }
                } else {
                    v
                }
            })
            .collect();
    }

    Some(ClassMember::Event(event))
}

fn transform_class_member(dump: &Dump, class: &Class, member: &ClassMember) -> Option<ClassMember> {
    if !is_valid_class_member(class, member) {
        return None;
    }

    match member {
        ClassMember::Property(property) => transform_class_property(dump, property),
        ClassMember::Function(function) => transform_class_function(dump, class, function),
        ClassMember::Event(event) => transform_class_event(dump, class, event),
        _ => None,
    }
}

fn transform_class(dump: &Dump, class: Class) -> Option<Class> {
    if should_generate_class(dump, &class) {
        Some(Class {
            members: class
                .members
                .iter()
                .filter_map(|member| transform_class_member(dump, &class, member))
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
    };

    for class in dump.classes.iter().cloned() {
        if let Some(class) = transform_class(dump, class) {
            new_dump.classes.push(class);
        }
    }

    new_dump
}
