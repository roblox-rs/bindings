use serde::{Deserialize, Deserializer};

use std::{collections::HashSet, hash::Hash};

fn set_from_vec<'de, T, D>(deserializer: D) -> Result<HashSet<T>, D::Error>
where
    T: Deserialize<'de> + Eq + Hash,
    D: Deserializer<'de>,
{
    let s: Vec<T> = Deserialize::deserialize(deserializer)?;
    Ok(s.into_iter().collect())
}

fn superclass_from_root_or_class_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(if s != "<<<ROOT>>>" { Some(s) } else { None })
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<Class>,
    pub enums: Vec<Enum>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub name: String,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<ClassTag>,

    #[serde(deserialize_with = "superclass_from_root_or_class_name")]
    pub superclass: Option<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ClassTag {
    Deprecated,
    NotBrowsable,
    NotCreatable,
    NotReplicated,
    PlayerReplicated,
    Service,
    Settings,
    UserSettings,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "MemberType")]
pub enum ClassMember {
    Property(ClassPropertyMember),
    Function(ClassFunctionMember),
    Event(ClassEventMember),
    Callback(ClassCallbackMember),
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassPropertyMember {
    pub name: String,
    pub security: Security,
    pub thread_safety: ThreadSafety,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<ClassMemberTag>,

    pub serialization: Serialization,
    pub value_type: ValueType,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct Serialization {
    pub can_load: bool,
    pub can_save: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFunctionMember {
    pub name: String,
    pub security: Security,
    pub thread_safety: ThreadSafety,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<ClassMemberTag>,

    pub parameters: Vec<ClassFunctionParameter>,
    pub return_type: ValueType,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFunctionParameter {
    pub name: String,
    #[serde(rename = "Type")]
    pub value_type: ValueType,
    pub default: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassEventMember {
    pub name: String,
    pub security: Security,
    pub thread_safety: ThreadSafety,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<ClassMemberTag>,

    pub parameters: Vec<ClassFunctionParameter>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassCallbackMember {
    pub name: String,
    pub security: Security,
    pub thread_safety: ThreadSafety,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<ClassMemberTag>,

    pub parameters: Vec<ClassFunctionParameter>,
    pub return_type: ValueType,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ClassMemberTag {
    CanYield,
    CustomLuaState,
    Deprecated,
    Hidden,
    NotBrowsable,
    NotReplicated,
    NotScriptable,
    NoYield,
    ReadOnly,
    Yields,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Security {
    Uniform(SecurityLevel),
    NonUniform(NonUniformSecurity),
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct NonUniformSecurity {
    pub read: SecurityLevel,
    pub write: SecurityLevel,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SecurityLevel {
    None,
    LocalUserSecurity,
    PluginSecurity,
    RobloxScriptSecurity,
    RobloxSecurity,
    NotAccessibleSecurity,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ThreadSafety {
    Unsafe,
    ReadSafe,
    Safe,
}

// TODO: handle optional types better?

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Category", content = "Name")]
pub enum ValueType {
    Primitive(PrimitiveKind),
    DataType(DataTypeKind),
    Class(String),
    Enum(String),
    Group(GroupKind),
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveKind {
    Void,
    Bool,
    Int,
    Int64,
    Float,
    Double,
    String,

    #[serde(alias = "int?")]
    OptionalInt,
    #[serde(rename = "float?")]
    OptionalFloat,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DataTypeKind {
    Axes,
    BinaryString,
    BrickColor,
    CatalogSearchParams,
    CFrame,
    Color3,
    ColorSequence,
    Content,
    DateTime,
    DockWidgetPluginGuiInfo,
    Faces,
    FloatCurveKey,
    Font,
    Function,
    NumberRange,
    NumberSequence,
    Objects,
    OverlapParams,
    PhysicalProperties,
    ProtectedString,
    QDir,
    QFont,
    Ray,
    RaycastParams,
    RaycastResult,
    #[serde(rename = "RBXScriptConnection")]
    RbxScriptConnection,
    #[serde(rename = "RBXScriptSignal")]
    RbxScriptSignal,
    Rect,
    Region3,
    Region3int16,
    RotationCurveKey,
    TweenInfo,
    UDim,
    UDim2,
    Vector2,
    // Vector2int16
    Vector3,
    Vector3int16,

    #[serde(rename = "CoordinateFrame?")]
    OptionalCFrame,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GroupKind {
    Array,
    Dictionary,
    Map,
    Tuple,
    Variant,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Enum {
    pub name: String,
    pub items: Vec<EnumItem>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EnumItem {
    pub name: String,
    #[serde(default)]
    pub legacy_names: Vec<String>,
    pub value: i32,
}

impl ClassMember {
    pub fn name(&self) -> &str {
        match self {
            ClassMember::Property(property) => &property.name,
            ClassMember::Function(function) => &function.name,
            ClassMember::Event(event) => &event.name,
            ClassMember::Callback(callback) => &callback.name,
        }
    }

    pub fn security(&self) -> &Security {
        match self {
            ClassMember::Property(property) => &property.security,
            ClassMember::Function(function) => &function.security,
            ClassMember::Event(event) => &event.security,
            ClassMember::Callback(callback) => &callback.security,
        }
    }

    #[allow(dead_code)]
    pub fn thread_safety(&self) -> &ThreadSafety {
        match self {
            ClassMember::Property(property) => &property.thread_safety,
            ClassMember::Function(function) => &function.thread_safety,
            ClassMember::Event(event) => &event.thread_safety,
            ClassMember::Callback(callback) => &callback.thread_safety,
        }
    }

    pub fn tags(&self) -> &HashSet<ClassMemberTag> {
        match self {
            ClassMember::Property(property) => &property.tags,
            ClassMember::Function(function) => &function.tags,
            ClassMember::Event(event) => &event.tags,
            ClassMember::Callback(callback) => &callback.tags,
        }
    }
}

impl From<ClassPropertyMember> for ClassMember {
    fn from(property: ClassPropertyMember) -> Self {
        ClassMember::Property(property)
    }
}

impl From<ClassFunctionMember> for ClassMember {
    fn from(function: ClassFunctionMember) -> Self {
        ClassMember::Function(function)
    }
}

impl From<ClassEventMember> for ClassMember {
    fn from(event: ClassEventMember) -> Self {
        ClassMember::Event(event)
    }
}

impl From<ClassCallbackMember> for ClassMember {
    fn from(callback: ClassCallbackMember) -> Self {
        ClassMember::Callback(callback)
    }
}

impl Dump {
    pub fn class(&self, class: &str) -> Option<&Class> {
        self.classes.iter().find(|value| value.name == class)
    }

    pub fn parent(&self, class: &Class) -> Option<&Class> {
        class
            .superclass
            .as_ref()
            .and_then(|superclass| self.class(superclass))
    }
}
