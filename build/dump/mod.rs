pub mod types;

use convert_case::{Case, Casing};
use types::{
    Class, ClassEventMember, ClassFunctionMember, ClassMember, ClassPropertyMember, DataTypeKind,
    Dump, PrimitiveKind, Security, ValueType,
};

use crate::{
    codegen::structs::{
        implementations, CodegenKind, InstanceMetadata, Member, MemberFlags, Namespace,
        NamespaceKind, Parameter,
    },
    config::{CLASS_BLACKLIST, DEPRECATED_APIS},
};

use self::types::ClassCallbackMember;

enum SecurityKind {
    Read,
    Write,
    Either,
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

pub fn to_snake(name: &str) -> String {
    name.from_case(Case::Pascal).to_case(Case::Snake)
}

pub fn is_camel_case(name: &str) -> bool {
    name.chars().next().unwrap().is_lowercase()
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

fn is_valid_class_member(class: &Class, member: &ClassMember) -> bool {
    if !is_security_accessible(member.security(), &SecurityKind::Either) {
        return false;
    }

    if member.tags().contains("NotScriptable") {
        return false;
    }

    // Remove deprecated members at least temporarily
    if member.tags().contains("Deprecated") && !DEPRECATED_APIS {
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

fn transform_value_type(value_type: &ValueType, is_getter: bool) -> CodegenKind {
    match value_type {
        ValueType::Primitive(kind) => match kind {
            PrimitiveKind::String => CodegenKind::String,
            PrimitiveKind::Void => CodegenKind::Void,
            PrimitiveKind::Bool => CodegenKind::Bool,
            PrimitiveKind::Int => CodegenKind::Number,
            PrimitiveKind::Int64 => CodegenKind::Number,
            PrimitiveKind::Float => CodegenKind::Number,
            PrimitiveKind::Double => CodegenKind::Number,
            PrimitiveKind::PossibleFloat => CodegenKind::Optional(Box::new(CodegenKind::Number)),
        },
        ValueType::DataType(kind) => match kind {
            DataTypeKind::PossibleCFrame => {
                CodegenKind::Optional(Box::new(CodegenKind::DataType("CFrame".to_string())))
            }
            _ => CodegenKind::DataType(format!("{kind:?}")),
        },
        ValueType::Class(kind) if is_getter => {
            CodegenKind::Optional(Box::new(CodegenKind::Instance(kind.clone())))
        }
        ValueType::Class(kind) => CodegenKind::Instance(kind.clone()),
        ValueType::Group(_) => todo!(),
        ValueType::Enum(kind) => CodegenKind::Enum(kind.clone()),
    }
}

fn transform_class_property(dump: &Dump, property: &ClassPropertyMember) -> Vec<Member> {
    if !is_type_generated(dump, &property.value_type) {
        return Vec::new();
    }

    let mut interfaces: Vec<Member> = Vec::new();
    interfaces.push(Member {
        flags: MemberFlags {
            deprecated: property.tags.contains("Deprecated"),
            ..MemberFlags::default()
        },
        implementation: implementations::PropertyGetter.into(),
        name: to_snake(&property.name),
        api_name: property.name.to_string(),
        inputs: vec![Parameter::new(
            "self",
            CodegenKind::Instance("Instance".to_string()),
        )],
        outputs: vec![transform_value_type(&property.value_type, true)],
    });

    if !property.tags.contains("ReadOnly") {
        interfaces.push(Member {
            flags: MemberFlags {
                deprecated: property.tags.contains("Deprecated"),
                ..MemberFlags::default()
            },
            implementation: implementations::PropertySetter.into(),
            name: format!("set_{}", to_snake(&property.name)),
            api_name: property.name.to_string(),
            inputs: vec![
                Parameter::new("self", CodegenKind::Instance("Instance".to_string())),
                Parameter::new("value", transform_value_type(&property.value_type, false)),
            ],
            outputs: vec![],
        });
    }

    interfaces
}

fn transform_class_function(
    dump: &Dump,
    class: &Class,
    function: &ClassFunctionMember,
) -> Option<Member> {
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

    let mut inputs = function
        .parameters
        .iter()
        .map(|v| {
            Parameter::new(
                &to_snake(&v.name),
                transform_value_type(&v.value_type, false),
            )
        })
        .collect::<Vec<_>>();

    inputs.insert(
        0,
        Parameter::new("self", CodegenKind::Instance("Instance".to_string())),
    );

    let will_collide = class
        .members
        .iter()
        .filter(|value| value.name() != function.name && !is_camel_case(value.name()))
        .any(|value| {
            let function_name = to_snake(&function.name);
            let member_name = to_snake(value.name());
            function_name == member_name
                || format!("set_{member_name}") == function_name
                || function_name == "clone"
        });

    Some(Member {
        flags: MemberFlags {
            deprecated: function.tags.contains("Deprecated"),
            ..MemberFlags::default()
        },
        implementation: implementations::Method.into(),
        api_name: function.name.clone(),
        name: if will_collide {
            format!("fn_{}", to_snake(&function.name))
        } else {
            to_snake(&function.name)
        },
        inputs,
        outputs: vec![transform_value_type(&function.return_type, true)],
    })
}

fn transform_class_event(dump: &Dump, event: &ClassEventMember) -> Option<Member> {
    if !event
        .parameters
        .iter()
        .all(|parameter| is_type_generated(dump, &parameter.value_type))
    {
        return None;
    }

    let inputs = event
        .parameters
        .iter()
        .map(|v| {
            Parameter::new(
                &to_snake(&v.name),
                transform_value_type(&v.value_type, true),
            )
        })
        .collect::<Vec<_>>();

    Some(Member {
        flags: MemberFlags {
            deprecated: event.tags.contains("Deprecated"),
            ..MemberFlags::default()
        },
        implementation: implementations::Event.into(),
        api_name: event.name.clone(),
        name: format!("on_{}", to_snake(&event.name)),
        inputs: vec![
            Parameter::new("self", CodegenKind::Instance("Instance".to_string())),
            Parameter::new(
                "callback",
                CodegenKind::Function(inputs, Box::new(CodegenKind::Void)),
            ),
        ],
        outputs: vec![CodegenKind::DataType("RbxScriptConnection".to_string())],
    })
}

fn transform_class_callback(dump: &Dump, callback: &ClassCallbackMember) -> Option<Member> {
    if !callback
        .parameters
        .iter()
        .all(|parameter| is_type_generated(dump, &parameter.value_type))
    {
        return None;
    }

    if !is_type_generated(dump, &callback.return_type) {
        return None;
    }

    let inputs = callback
        .parameters
        .iter()
        .map(|v| {
            Parameter::new(
                &to_snake(&v.name),
                transform_value_type(&v.value_type, true),
            )
        })
        .collect::<Vec<_>>();

    Some(Member {
        flags: MemberFlags {
            deprecated: callback.tags.contains("Deprecated"),
            ..MemberFlags::default()
        },
        implementation: implementations::Callback.into(),
        name: format!("on_{}", to_snake(&callback.name)),
        api_name: callback.name.clone(),
        inputs: vec![
            Parameter::new("self", CodegenKind::Instance("Instance".to_string())),
            Parameter::new(
                "callback",
                CodegenKind::Function(
                    inputs,
                    Box::new(transform_value_type(&callback.return_type, false)),
                ),
            ),
        ],
        outputs: Vec::new(),
    })
}

fn transform_class_member(dump: &Dump, class: &Class, member: &ClassMember) -> Vec<Member> {
    if !is_valid_class_member(class, member) {
        return vec![];
    }

    match member {
        ClassMember::Event(event) => transform_class_event(dump, event).into_iter().collect(),
        ClassMember::Property(property) => transform_class_property(dump, property),
        ClassMember::Function(function) => transform_class_function(dump, class, function)
            .into_iter()
            .collect(),
        ClassMember::Callback(callback) => transform_class_callback(dump, callback)
            .into_iter()
            .collect(),
    }
}

fn transform_class(dump: &Dump, class: Class) -> Option<Namespace> {
    if should_generate_class(dump, &class) {
        Some(Namespace {
            kind: NamespaceKind::Instance(InstanceMetadata {
                creatable: !class.tags.contains("NotCreatable"),
                service: class.tags.contains("Service"),
            }),
            name: class.name.clone(),
            extends: if class.superclass == "<<<ROOT>>>" {
                None
            } else {
                Some(class.superclass.clone())
            },
            members: class
                .members
                .iter()
                .flat_map(|member| {
                    let mut members = transform_class_member(dump, &class, member);
                    for member in &mut members {
                        member.outputs.retain(|v| !matches!(v, CodegenKind::Void))
                    }
                    members
                })
                .collect(),
        })
    } else {
        None
    }
}

pub fn transform_dump(dump: &Dump) -> Vec<Namespace> {
    dump.classes
        .iter()
        .cloned()
        .flat_map(|value| transform_class(dump, value))
        .collect()
}
