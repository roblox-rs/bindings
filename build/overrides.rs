use convert_case::{Case, Casing};

use crate::codegen::structs::{implementations::*, CodegenKind, Member, MemberFlags, Parameter};

pub struct PartialNamespace {
    /// Name of this namespace
    pub name: String,

    /// Members of this namespace
    pub members: Vec<Member>,
}

pub fn get_instances() -> Vec<PartialNamespace> {
    let key_code = r#enum!(KeyCode);

    let instance = instance!(Instance);
    let player = instance!(Player);

    let unknown = CodegenKind::Unknown;
    let string = CodegenKind::String;
    let void = CodegenKind::Void;

    namespaces! {
        Instance;

        struct Instance {
            field!(Parent: optional!(instance)),

            event!(Changed::on_instance_changed(property: string)),
        };

        struct BasePart {
            event!(Touched::on_touched(part: instance)),
        };

        struct Humanoid {
            event!(Touched::on_humanoid_touched(part: instance, limb: instance)),
        };

        struct RemoteEvent {
            event!(OnServerEvent(player: player, values: tuple!(unknown))),
            event!(OnClientEvent(values: tuple!(unknown))),

            method!(FireAllClients(values: tuple!(unknown)) -> void),
            method!(FireClient(player: player, values: tuple!(unknown)) -> void),
            method!(FireServer(values: tuple!(unknown)) -> void),
        };

        struct RemoteFunction {
            callback!(OnServerInvoke(player: player, values: tuple!(unknown)) -> tuple!(unknown)),
            callback!(OnClientInvoke(values: tuple!(unknown)) -> tuple!(unknown)),

            method!(InvokeServer(values: tuple!(unknown)) -> tuple!(unknown)),
            method!(InvokeClient(player: player, values: tuple!(unknown)) -> tuple!(unknown)),
        };

        struct BindableEvent {
            event!(Event(values: tuple!(unknown))),

            method!(Fire(values: tuple!(unknown)) -> void),
        };

        struct UserInputService {
            method!(GetKeysPressed() -> array!(key_code)),
        };

        struct CollectionService {
            method!(GetTags(instance: instance) -> array!(string)),
        };
    }
}

pub fn get_datatypes() -> Vec<PartialNamespace> {
    let normal_id = r#enum!(NormalId);
    let axis = r#enum!(Axis);

    let vector3 = datatype!(Vector3);
    let cframe = datatype!(CFrame);
    let color3 = datatype!(Color3);

    let number = CodegenKind::Number;
    let bool = CodegenKind::Bool;

    namespaces! {
        DataType;

        struct Vector3 {
            method!(static new::new() -> vector3),
            method!(static new::new_with_position(x: number, y: number, z: number) -> vector3),
            method!(static new::new_with_vector3(v3: vector3) -> vector3),
            method!(static new::new_with_normal(normal: normal_id) -> vector3),
            method!(static new::new_with_axis(axis: axis) -> vector3),

            field!(static zero: vector3),
            field!(static one: vector3),
            field!(static xAxis: vector3),
            field!(static yAxis: vector3),
            field!(static zAxis: vector3),

            field!(readonly X: number),
            field!(readonly Y: number),
            field!(readonly Z: number),
            field!(readonly Magnitude: number),
            field!(readonly Unit: vector3),

            method!(Lerp(goal: vector3, alpha: number) -> vector3),
            method!(Cross(other: vector3) -> vector3),
            method!(Angle(other: vector3, axis: vector3) -> number),
            method!(Dot(other: vector3) -> number),
            method!(FuzzyEq(other: vector3, epsilon: number) -> bool),
            method!(Max(others: tuple!(vector3)) -> vector3),
            method!(Min(others: tuple!(vector3)) -> vector3),

            operator!(Add(+ vector3 => vector3)),
            operator!(Sub(- vector3 => vector3)),
            operator!(Div(/ vector3 => vector3)),
            operator!(Div(/ number => vector3)),
            operator!(Mul(* vector3 => vector3)),
            operator!(Mul(* number => vector3)),
        };

        struct CFrame {
            method!(static new::new() -> cframe),
            method!(static new::new_with_position(x: number, y: number, z: number) -> cframe),

            field!(readonly Position: vector3),
            field!(readonly Rotation: cframe),
            field!(readonly LookVector: vector3),
            field!(readonly RightVector: vector3),
            field!(readonly UpVector: vector3),
            field!(readonly XVector: vector3),
            field!(readonly YVector: vector3),
            field!(readonly ZVector: vector3),
            field!(readonly X: number),
            field!(readonly Y: number),
            field!(readonly Z: number),

            method!(Inverse() -> cframe),
            method!(Lerp(goal: cframe, alpha: number) -> cframe),
            method!(Orthonormalize() -> cframe),
            method!(ToWorldSpace(cf: cframe) -> cframe),
            method!(ToObjectSpace(cf: cframe) -> cframe),
            method!(PointToWorldSpace(v3: vector3) -> vector3),
            method!(PointToObjectSpace(v3: vector3) -> vector3),
            method!(VectorToWorldSpace(v3: vector3) -> vector3),
            method!(VectorToObjectSpace(v3: vector3) -> vector3),
            method!(GetComponents() -> tuple!(number)),

            operator!(Add(+ vector3 => cframe)),
            operator!(Sub(- vector3 => cframe)),
            operator!(Mul(* cframe => cframe)),
            operator!(Mul(* vector3 => vector3)),
        };

        struct Color3 {
            method!(static fromRGB(r: number, g: number, b: number) -> color3),
        };
    }
}

pub fn internal_namespace() -> PartialNamespace {
    let unknown = CodegenKind::Unknown;
    let number = CodegenKind::Number;
    let string = CodegenKind::String;

    PartialNamespace {
        name: "Internal".to_string(),
        members: vec![
            member! {
                string_to_lua_value(PointerConversion);

                signature = (string: string) -> unknown;
            },
            member! {
                float_to_lua_value(PointerConversion);

                signature = (float: number) -> unknown;
            },
            member! {
                lua_value_to_string(PrimitiveConversion("string"));

                signature = (value: unknown) -> optional!(string);
            },
            member! {
                lua_value_to_float(PrimitiveConversion("number"));

                signature = (value: unknown) -> optional!(number);
            },
            member! {
                new::instance_new(StaticFunction(Some("Instance"), None));

                signature = (class_name: string) -> instance!(Instance);
            },
        ],
    }
}

macro_rules! parse_member_opts {
    ($member:expr; signature = ($($name:ident : $kind:expr),*) -> ($($output:expr),*); $($rest:tt)*) => {
        $member.inputs = vec![$(Parameter::new(stringify!($name), $kind.clone())),*];
        $member.outputs = vec![$($output.clone()),*];
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; signature = ($($name:ident : $kind:expr),*) -> $output:expr; $($rest:tt)*) => {
        $member.inputs = vec![$(Parameter::new(stringify!($name), $kind.clone())),*];
        $member.outputs = vec![$output.clone()];
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; flags = $flags:expr; $($rest:tt)*) => {
        $member.flags = $flags;
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr;) => {}
}

macro_rules! member {
    ($name:ident($impl:expr); $($rest:tt)*) => {
        member!($name::$name($impl); $($rest)*)
    };
    ($api_name:ident::$name:ident($impl:expr); $($rest:tt)*) => {{
        let mut member = Member {
            flags: MemberFlags::default(),
            implementation: $impl.into(),
            name: stringify!($name).to_string(),
            api_name: stringify!($api_name).to_string(),
            inputs: vec![],
            outputs: vec![],
        };
        parse_member_opts!(member; $($rest)*);
        member
    }};
}

macro_rules! event {
    ($name:ident($($parameter:ident : $kind:expr),*)) => {
        event!($name::$name($($parameter:$kind),*))
    };
    ($api_name:ident::$name:ident($($parameter:ident : $kind:expr),*)) => {
        vec![Member {
            flags: MemberFlags::default(),
            implementation: Event.into(),
            api_name: stringify!($api_name).to_string(),
            name: stringify!($name).to_case(Case::Snake),
            inputs: vec![
                Parameter::new("self", CodegenKind::Unknown),
                Parameter::new("callback", CodegenKind::Function(
                    vec![$(Parameter::new(stringify!($parameter), $kind.clone())),*],
                    Box::new(CodegenKind::Void),
                ))
            ],
            outputs: vec![CodegenKind::DataType("RbxScriptConnection".to_string())],
        }]
    };
}

macro_rules! callback {
    ($name:ident($($parameter:ident : $kind:expr),*) -> $result:expr) => {
        callback!($name::$name($($parameter:$kind),*) -> $result)
    };
    ($api_name:ident::$name:ident($($parameter:ident : $kind:expr),*) -> $result:expr) => {
        vec![Member {
            flags: MemberFlags::default(),
            implementation: Callback.into(),
            api_name: stringify!($api_name).to_string(),
            name: stringify!($name).to_case(Case::Snake),
            inputs: vec![
                Parameter::new("self", CodegenKind::Unknown),
                Parameter::new("callback", CodegenKind::Function(
                    vec![$(Parameter::new(stringify!($parameter), $kind.clone())),*],
                    Box::new($result.clone()),
                )),
            ],
            outputs: vec![],
        }]
    };
}

macro_rules! method {
    (static $name:ident($($parameter:ident : $kind:expr),*) -> $result:expr) => {
        method!(static $name::$name($($parameter: $kind),*) -> $result)
    };

    (static $api_name:ident::$name:ident($($parameter:ident : $kind:expr),*) -> $result:expr) => {
        vec![Member {
            flags: MemberFlags::default(),
            implementation: StaticFunction(None, Some(stringify!($api_name))).into(),
            api_name: stringify!($api_name).to_string(),
            name: stringify!($name).to_case(Case::Snake),
            inputs: vec![$(Parameter::new(stringify!($parameter), $kind.clone())),*],
            outputs: vec![$result.clone()],
        }]
    };

    ($name:ident($($parameter:ident : $kind:expr),*) -> $result:expr) => {
        vec![Member {
            flags: MemberFlags::default(),
            implementation: Method.into(),
            api_name: stringify!($name).to_string(),
            name: stringify!($name).to_case(Case::Snake),
            inputs: vec![
                Parameter::new("self", CodegenKind::Unknown),
                $(Parameter::new(stringify!($parameter), $kind.clone())),*
            ],
            outputs: vec![$result.clone()],
        }]
    };
}

macro_rules! operator {
    ($trait:ident($op:tt $rhs:expr => $output:expr)) => {
        vec![Member {
            flags: MemberFlags {
                operator: Some(stringify!($trait)),
                ..MemberFlags::default()
            },
            implementation: BinOp(stringify!($op)).into(),
            api_name: format!("{}_{}", stringify!($trait), stringify!($rhs)),
            name: stringify!($trait).to_case(Case::Snake),
            inputs: vec![
                Parameter::new("self", CodegenKind::Unknown),
                Parameter::new("value", $rhs.clone()),
            ],
            outputs: vec![$output.clone()],
        }]
    };
}

macro_rules! field {
    (static $field:ident : $kind:expr) => {
        vec![Member {
            flags: MemberFlags::default(),
            implementation: StaticProperty(None, Some(stringify!($field))).into(),
            api_name: stringify!($field).to_string(),
            name: stringify!($field).to_case(Case::Snake),
            inputs: vec![],
            outputs: vec![$kind.clone()],
        }]
    };
    (readonly $field:ident : $kind:expr) => {{
        vec![Member {
            flags: MemberFlags::default(),
            implementation: PropertyGetter.into(),
            api_name: stringify!($field).to_string(),
            name: stringify!($field).to_case(Case::Snake),
            inputs: vec![Parameter::new("self", CodegenKind::Unknown)],
            outputs: vec![$kind.clone()],
        }]
    }};
    ($field:ident : $kind:expr) => {{
        vec![
            Member {
                flags: MemberFlags::default(),
                implementation: PropertyGetter.into(),
                api_name: stringify!($field).to_string(),
                name: stringify!($field).to_case(Case::Snake),
                inputs: vec![Parameter::new("self", CodegenKind::Unknown)],
                outputs: vec![$kind.clone()],
            },
            Member {
                flags: MemberFlags::default(),
                implementation: PropertySetter.into(),
                api_name: stringify!($field).to_string(),
                name: format!("set_{}", stringify!($field).to_case(Case::Snake)),
                inputs: vec![
                    Parameter::new("self", CodegenKind::Unknown),
                    Parameter::new("value", $kind.clone()),
                ],
                outputs: vec![],
            },
        ]
    }};
}

macro_rules! namespaces {
	($kind:ident; $(struct $name:ident { $($member:expr,)* };)*) => {
        vec![$(
            PartialNamespace {
                name: stringify!($name).to_string(),
                members: vec![
                    $($member),*
                ]
                .into_iter()
                .flatten()
                .map(|mut v| {
                    if let Some(mut input) = v.inputs.get_mut(0) {
                        if input.name == "self" {
                            input.kind = CodegenKind::$kind(stringify!($name).to_string());
                        }
                    }
                    v
                })
                .collect()
            }
        ),*]
	};
}

macro_rules! tuple {
    ($kind:expr) => {
        CodegenKind::Tuple(Box::new($kind.clone()))
    };
}

macro_rules! datatype {
    ($kind:ident) => {
        CodegenKind::DataType(stringify!($kind).to_string())
    };
}

macro_rules! instance {
    ($kind:ident) => {
        CodegenKind::Instance(stringify!($kind).to_string())
    };
}

macro_rules! array {
    ($kind:expr) => {
        CodegenKind::Vec(Box::new($kind.clone()))
    };
}

macro_rules! optional {
    ($kind:expr) => {
        CodegenKind::Optional(Box::new($kind.clone()))
    };
}

macro_rules! r#enum {
    ($kind:ident) => {
        CodegenKind::Enum(stringify!($kind).to_string())
    };
}

use {
    array, callback, datatype, event, field, instance, member, method, namespaces, operator,
    optional, parse_member_opts, r#enum, tuple,
};
