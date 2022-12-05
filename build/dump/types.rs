use serde::{Deserialize, Deserializer};

use std::collections::HashSet;

fn set_from_vec<'de, D>(deserializer: D) -> Result<HashSet<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(s.into_iter().collect())
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<Class>,
    pub enums: Vec<Enum>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enum {
    pub items: Vec<EnumItem>,
    pub name: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnumItem {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub name: String,
    pub superclass: String,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "MemberType")]
pub enum ClassMember {
    Function(ClassFunctionMember),
    Property(ClassPropertyMember),
    Callback(ClassCallbackMember),
    Event(ClassEventMember),
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassEventMember {
    pub name: String,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<String>,
    pub security: Security,
    pub parameters: Vec<ClassFunctionParameter>,

    /// The real Roblox event name.
    /// This differs from name in cases like `Instance.Changed`
    pub event_name: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassCallbackMember {
    pub name: String,
    pub return_type: ValueType,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<String>,
    pub security: Security,
    pub parameters: Vec<ClassFunctionParameter>,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFunctionMember {
    pub name: String,
    pub return_type: ValueType,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<String>,
    pub security: Security,
    pub parameters: Vec<ClassFunctionParameter>,

    /// The real Roblox function name.
    /// This differs from name in case of collisions like SetAxis() and .Axis =
    pub func_name: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFunctionParameter {
    pub name: String,
    #[serde(rename = "Type")]
    pub value_type: ValueType,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassPropertyMember {
    pub name: String,
    pub value_type: ValueType,
    #[serde(deserialize_with = "set_from_vec")]
    #[serde(default)]
    pub tags: HashSet<String>,
    pub security: Security,
}

#[derive(Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum Security {
    Uniform(String),
    NonUniform(NonUniformSecurity),
}

#[derive(Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NonUniformSecurity {
    pub read: String,
    pub write: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "Category", content = "Name")]
pub enum ValueType {
    Primitive(PrimitiveKind),
    DataType(DataTypeKind),
    Class(String),
    Group(String),
    Enum(String),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum DataTypeKind {
    Function,
    CatalogSearchParams,
    RaycastParams,
    DockWidgetPluginGuiInfo,
    OverlapParams,
    Vector3int16,
    Region3,
    Vector3,
    Vector2,
    Ray,
    #[serde(rename = "RBXScriptSignal")]
    RbxScriptSignal,
    #[serde(rename = "RBXScriptConnection")]
    RbxScriptConnection,
    Objects,
    Rect,
    Axes,
    UDim2,
    Faces,
    CFrame,
    RaycastResult,
    ProtectedString,
    RotationCurveKey,
    NumberRange,
    Region3int16,
    PhysicalProperties,
    BinaryString,
    Content,
    Color3,
    BrickColor,
    ColorSequence,
    NumberSequence,
    FloatCurveKey,
    Font,
    QDir,
    QFont,
    DateTime,
    TweenInfo,
    UDim,
    #[serde(rename = "CoordinateFrame?")]
    PossibleCFrame,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveKind {
    String,
    Void,
    Bool,
    Int,
    Int64,
    Float,
    Double,
    #[serde(rename = "float?")]
    #[serde(alias = "int?")]
    PossibleFloat,
}

impl ClassMember {
    pub fn tags(&self) -> &HashSet<String> {
        match self {
            ClassMember::Event(event) => &event.tags,
            ClassMember::Function(function) => &function.tags,
            ClassMember::Property(property) => &property.tags,
            ClassMember::Callback(callback) => &callback.tags,
        }
    }

    pub fn security(&self) -> &Security {
        match self {
            ClassMember::Event(event) => &event.security,
            ClassMember::Function(function) => &function.security,
            ClassMember::Property(property) => &property.security,
            ClassMember::Callback(callback) => &callback.security,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ClassMember::Event(event) => &event.name,
            ClassMember::Function(function) => &function.name,
            ClassMember::Property(property) => &property.name,
            ClassMember::Callback(callback) => &callback.name,
        }
    }
}

impl From<ClassCallbackMember> for ClassMember {
    fn from(callback: ClassCallbackMember) -> Self {
        ClassMember::Callback(callback)
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

impl Dump {
    pub fn class(&self, class: &str) -> &Class {
        self.classes
            .iter()
            .find(|value| value.name == class)
            .unwrap()
    }

    pub fn parent(&self, class: &Class) -> Option<&Class> {
        self.classes.iter().find(|v| v.name == class.superclass)
    }
}
