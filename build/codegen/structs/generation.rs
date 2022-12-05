use convert_case::{Case, Casing};

use crate::codegen::UniqueIds;

use super::{implementations::Implementation, CodegenKind};

pub struct RenderData<'a> {
    pub namespace_name: &'a str,
    pub member_name: &'a str,

    pub namespace: &'a Namespace,
    pub member: &'a Member,
    pub arguments: &'a [String],
    pub parameters: &'a [Vec<String>],
    pub prereqs: &'a mut Vec<String>,
}

pub trait MemberImplementation {
    fn render(&self, data: RenderData) -> String;

    fn ffi_prefix(&self) -> &'static str {
        "custom"
    }
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub kind: CodegenKind,
}

impl Parameter {
    pub fn new(name: &str, kind: CodegenKind) -> Parameter {
        Parameter {
            name: name.to_string(),
            kind,
        }
    }

    pub fn with_kind(&self, kind: &CodegenKind) -> Parameter {
        Parameter::new(&self.name, kind.clone())
    }

    pub fn names(&self, ids: &UniqueIds) -> Vec<String> {
        let name = &self.name;
        let width = self.kind.width();
        if width == 1 {
            vec![ids.next_names(&[name])]
        } else {
            let mut parameters: Vec<String> = Vec::new();
            for i in 0..width {
                if let Some(describe) = self.kind.describe_width(i) {
                    parameters.push(ids.next_names(&[name, &describe]));
                } else {
                    parameters.push(ids.next_names(&[name]));
                }
            }
            parameters
        }
    }
}

#[derive(Default)]
pub struct MemberFlags {
    pub deprecated: bool,
    pub operator: Option<&'static str>,
}

pub struct Member {
    pub flags: MemberFlags,

    /// The implementation used to render the bindings
    pub implementation: Implementation,

    /// The name assigned by the bindings
    pub name: String,

    /// The real api name, also used for FFI
    pub api_name: String,

    /// The parameters to this function (on the Luau side)
    pub inputs: Vec<Parameter>,

    /// The outputs of this function (on the Luau side)
    pub outputs: Vec<CodegenKind>,
}

#[derive(Clone)]
pub struct InstanceMetadata {
    pub creatable: bool,
    pub service: bool,
}

#[derive(Clone)]
pub enum NamespaceKind {
    /// Describes a function namespace
    Functions,

    /// Describes an instance type
    Instance(InstanceMetadata),

    /// Describes a data type
    DataType,
}

pub struct Namespace {
    pub kind: NamespaceKind,

    /// The name of this namespace
    pub name: String,

    /// The namespace this namespace extends, if any
    pub extends: Option<String>,

    /// Members of this namespace
    pub members: Vec<Member>,
}

impl Namespace {
    pub fn ffi_name(&self, member: &Member) -> String {
        let namespace_name = self.name.to_case(Case::Snake);
        let member_name = member.name.to_case(Case::Snake);
        let api_name = member.api_name.to_case(Case::Snake);
        let prefix = member.implementation.ffi_prefix().to_case(Case::Snake);

        if api_name == member_name {
            format!("{prefix}_{namespace_name}_{member_name}")
        } else {
            format!("{prefix}_{namespace_name}_{api_name}_{member_name}")
        }
    }
}

#[derive(Copy, Clone)]
pub enum DeclarationContext {
    Function,
    Trait,
    TraitImpl,
    Extern,
}
