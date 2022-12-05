use super::{Parameter, TypeLayout};

#[derive(Clone, Debug)]
pub enum CodegenKind {
    Function(Vec<Parameter>, Box<CodegenKind>),
    Optional(Box<CodegenKind>),
    Tuple(Box<CodegenKind>),
    Vec(Box<CodegenKind>),
    Instance(String),
    DataType(String),
    Enum(String),
    Unknown,
    String,
    Number,
    Bool,
    Void,
}

impl CodegenKind {
    pub fn describe_width(&self, index: u32) -> Option<String> {
        match self {
            CodegenKind::Vec(_) | CodegenKind::Tuple(_) | CodegenKind::String => match index {
                0 => Some("addr".to_string()),
                1 => Some("len".to_string()),
                _ => unreachable!(),
            },
            CodegenKind::Function(_, _) => match index {
                0 => Some("data".to_string()),
                1 => Some("vtable".to_string()),
                _ => unreachable!(),
            },
            CodegenKind::Optional(kind) => match index {
                0 => Some("kind".to_string()),
                _ => kind.describe_width(index - 1).map(|v| format!("value_{v}")),
            },
            _ => None,
        }
    }

    pub fn layout(&self) -> TypeLayout {
        TypeLayout::new(self)
    }

    pub fn width(&self) -> u32 {
        self.layout().width()
    }
}
