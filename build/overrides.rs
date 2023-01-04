use convert_case::{Case, Casing};

use crate::codegen::structs::{
    implementations::*, Async, CodegenKind, Member, MemberFlags, Parameter,
};

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
        struct Instance: Instance {
            Parent: optional!(instance);

            event Changed::on_instance_changed(property: string);
        };

        struct BasePart: Instance {
            event Touched::on_touched(part: instance);
        };

        struct Humanoid: Instance {
            event Touched::on_humanoid_touched(part: instance, limb: instance);
        };

        struct RemoteEvent: Instance {
            event OnServerEvent(player: player, values: tuple!(unknown));
            event OnClientEvent(values: tuple!(unknown));

            FireAllClients(values: tuple!(unknown)) -> ();
            FireClient(player: player, values: tuple!(unknown)) -> ();
            FireServer(values: tuple!(unknown)) -> ();
        };

        struct RemoteFunction: Instance {
            callback OnServerInvoke(player: player, values: tuple!(unknown)) -> tuple!(unknown);
            callback OnClientInvoke(values: tuple!(unknown)) -> tuple!(unknown);

            InvokeServer(values: tuple!(unknown)) -> (tuple!(unknown));
            InvokeClient(player: player, values: tuple!(unknown)) -> (tuple!(unknown));
        };

        struct BindableEvent: Instance {
            event Event(values: tuple!(unknown));

            Fire(values: tuple!(unknown)) -> (void);
        };

        struct UserInputService: Instance {
            GetKeysPressed() -> (array!(key_code));
        };

        struct CollectionService: Instance {
            GetTags(instance: instance) -> (array!(string));
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
        struct Vector3: DataType {
            static new::new() -> (vector3);
            static new::new_with_position(x: number, y: number, z: number) -> (vector3);
            static new::new_with_vector3(v3: vector3) -> (vector3);
            static new::new_with_normal(normal: normal_id) -> (vector3);
            static new::new_with_axis(axis: axis) -> (vector3);

            static zero: vector3;
            static one: vector3;
            static xAxis: vector3;
            static yAxis: vector3;
            static zAxis: vector3;

            readonly X: number;
            readonly Y: number;
            readonly Z: number;
            readonly Magnitude: number;
            readonly Unit: vector3;

            Lerp(goal: vector3, alpha: number) -> (vector3);
            Cross(other: vector3) -> (vector3);
            Angle(other: vector3, axis: vector3) -> (number);
            Dot(other: vector3) -> (number);
            FuzzyEq(other: vector3, epsilon: number) -> (bool);
            Max(others: tuple!(vector3)) -> (vector3);
            Min(others: tuple!(vector3)) -> (vector3);

            operator Neg(-) -> vector3;
            operator Add(+ vector3) -> vector3;
            operator Sub(- vector3) -> vector3;
            operator Div(/ vector3) -> vector3;
            operator Div(/ number) -> vector3;
            operator Mul(* vector3) -> vector3;
            operator Mul(* number) -> vector3;
        };

        struct CFrame: DataType {
            static new::new() -> (cframe);
            static new::new_with_position(x: number, y: number, z: number) -> (cframe);

            readonly Position: vector3;
            readonly Rotation: cframe;
            readonly LookVector: vector3;
            readonly RightVector: vector3;
            readonly UpVector: vector3;
            readonly XVector: vector3;
            readonly YVector: vector3;
            readonly ZVector: vector3;
            readonly X: number;
            readonly Y: number;
            readonly Z: number;

            Inverse() -> (cframe);
            Lerp(goal: cframe, alpha: number) -> (cframe);
            Orthonormalize() -> (cframe);
            ToWorldSpace(cf: cframe) -> (cframe);
            ToObjectSpace(cf: cframe) -> (cframe);
            PointToWorldSpace(v3: vector3) -> (vector3);
            PointToObjectSpace(v3: vector3) -> (vector3);
            VectorToWorldSpace(v3: vector3) -> (vector3);
            VectorToObjectSpace(v3: vector3) -> (vector3);
            GetComponents() -> (tuple!(number));

            operator Add(+ vector3) -> cframe;
            operator Sub(- vector3) -> cframe;
            operator Mul(* cframe) -> cframe;
            operator Mul(* vector3) -> vector3;
        };

        struct Color3: DataType {
            static new(r: number, g: number, b: number) -> (color3);
            static fromRGB(r: number, g: number, b: number) -> (color3);
            static fromHSV(h: number, s: number, v: number) -> (color3);

            readonly R: number;
            readonly G: number;
            readonly B: number;
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

macro_rules! parse_namespace_fields {
    // Static properties
    (($namespace:expr;$kind:expr) static $api_name:ident::$name:ident: $type:expr; $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(StaticProperty(None, Some(stringify!($api_name))));

            signature = () -> $type;
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) static $name:ident: $type:expr; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) static $name::$name: $type; $($tt)*);
    };

    // Readonly properties
    (($namespace:expr;$kind:expr) readonly $api_name:ident::$name:ident: $type:expr; $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(PropertyGetter);

            signature = (self: $kind) -> $type;
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) readonly $name:ident: $type:expr; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) readonly $name::$name: $type; $($tt)*);
    };

    // Writable properties
    (($namespace:expr;$kind:expr) $api_name:ident::$name:ident: $type:expr; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) readonly $api_name::$name: $type;);

        $namespace.members.push(member! {
            $api_name(PropertySetter);

            name = format!("set_{}", stringify!($name).to_case(Case::Snake));
            signature = (self: $kind, value: $type) -> ();
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) $name:ident: $type:expr; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) $name::$name: $type; $($tt)*);
    };

    // Static methods
    (($namespace:expr;$kind:expr) static $api_name:ident::$name:ident($($parameter:ident : $type:expr),*) -> ($($result:expr),*); $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(StaticFunction(None, Some(stringify!($api_name))));

            signature = ($($parameter: $type),*) -> ($($result),*);
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) static $name:ident($($parameter:ident : $type:expr),*) -> $result:tt; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) static $name::$name($($parameter:$type),*) -> $result; $($tt)*);
    };

    // Methods
    (($namespace:expr;$kind:expr) $api_name:ident::$name:ident($($parameter:ident : $type:expr),*) -> ($($result:expr),*); $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(Method);

            signature = (self: $kind, $($parameter: $type),*) -> ($($result),*);
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) $name:ident($($parameter:ident : $type:expr),*) -> $result:tt; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) $name::$name($($parameter:$type),*) -> $result; $($tt)*);
    };

    // Events
    (($namespace:expr;$kind:expr) event $api_name:ident::$name:ident($($parameter:ident : $type:expr),*); $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(Event);

            signature = (
                self: $kind,
                callback: CodegenKind::Function(
                    vec![$(Parameter::new(stringify!($parameter), $type.clone())),*],
                    Box::new(CodegenKind::Void),
                    Async::No
                )
            ) -> (datatype!(RbxScriptConnection));
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) event $name:ident($($parameter:ident : $type:expr),*); $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) event $name::$name($($parameter:$type),*); $($tt)*);
    };

    // Callbacks
    (($namespace:expr;$kind:expr) callback $api_name:ident::$name:ident($($parameter:ident : $type:expr),*) -> $result:expr; $($tt:tt)*) => {
        $namespace.members.push(member! {
            $api_name::$name(Callback);

            signature = (
                self: $kind,
                callback: CodegenKind::Function(
                    vec![$(Parameter::new(stringify!($parameter), $type.clone())),*],
                    Box::new($result.clone()),
                    Async::Yes
                )
            ) -> ();
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) callback $name:ident($($parameter:ident : $type:expr),*) -> $result:expr; $($tt:tt)*) => {
        parse_namespace_fields!(($namespace;$kind) callback $name::$name($($parameter:$type),*) -> $result; $($tt)*);
    };

    // Operators
    (($namespace:expr;$kind:expr) operator $trait:ident($op:tt) -> $type:expr; $($tt:tt)*) => {
        $namespace.members.push(member! {
            $trait(UnOp(stringify!($op)));

            api_name = format!("{}", stringify!($trait));
            signature = (self: $kind) -> ($type);
            flags = MemberFlags {
                operator: Some(stringify!($trait)),
                ..MemberFlags::default()
            };
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };
    (($namespace:expr;$kind:expr) operator $trait:ident($op:tt $rhs:expr) -> $type:expr; $($tt:tt)*) => {
        $namespace.members.push(member! {
            $trait(BinOp(stringify!($op)));

            api_name = format!("{}_{}", stringify!($trait), stringify!($rhs));
            signature = (self: $kind, value: $rhs) -> ($type);
            flags = MemberFlags {
                operator: Some(stringify!($trait)),
                ..MemberFlags::default()
            };
        });

        parse_namespace_fields!(($namespace;$kind) $($tt)*);
    };

    // No more properties
    (($namespace:expr;$kind:expr)) => {};
}

macro_rules! namespaces {
    ($(struct $name:ident: $kind:ident { $($tt:tt)* };)*) => {
        vec![$(
            {
            let mut namespace = PartialNamespace {
                name: stringify!($name).to_string(),
                members: vec![],
            };
            #[allow(unused)]
            let kind = CodegenKind::$kind(stringify!($name).to_string());
            parse_namespace_fields!((namespace;kind) $($tt)*);
            namespace
        }
    ),*]
    };
}

macro_rules! parse_member_opts {
    ($member:expr; signature = ($($name:ident : $kind:expr),*$(,)?) -> ($($output:expr),*); $($rest:tt)*) => {
        $member.inputs = vec![$(Parameter::new(stringify!($name), $kind.clone())),*];
        $member.outputs = vec![$($output.clone()),*];
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; signature = ($($name:ident : $kind:expr),*$(,)?) -> $output:expr; $($rest:tt)*) => {
        $member.inputs = vec![$(Parameter::new(stringify!($name), $kind.clone())),*];
        $member.outputs = vec![$output.clone()];
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; flags = $flags:expr; $($rest:tt)*) => {
        $member.flags = $flags;
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; name = $name:expr; $($rest:tt)*) => {
        $member.name = $name;
        parse_member_opts!($member; $($rest)*);
    };
    ($member:expr; api_name = $api_name:expr; $($rest:tt)*) => {
        $member.api_name = $api_name;
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
            name: stringify!($name).to_case(Case::Snake),
            api_name: stringify!($api_name).to_string(),
            inputs: vec![],
            outputs: vec![],
        };
        parse_member_opts!(member; $($rest)*);
        member
    }};
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
    array, datatype, instance, member, namespaces, optional, parse_member_opts,
    parse_namespace_fields, r#enum, tuple,
};
