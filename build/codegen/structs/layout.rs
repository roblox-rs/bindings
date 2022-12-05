use crate::config::MULTI_VALUE_SUPPORT;

use super::CodegenKind;

pub enum Representation {
    /// u32 / usize
    Pointer,

    /// u8
    Byte,

    /// f64
    Float,
}

pub struct TypeLayout {
    representation: Vec<Representation>,
}

impl TypeLayout {
    fn represent(kind: &CodegenKind) -> Vec<Representation> {
        match kind {
            CodegenKind::Optional(kind) => {
                let mut representations = vec![Representation::Pointer];
                representations.extend(TypeLayout::represent(kind));
                representations
            }
            CodegenKind::Function(_, _) => vec![Representation::Pointer, Representation::Pointer],
            CodegenKind::Instance(_) => vec![Representation::Pointer],
            CodegenKind::DataType(_) => vec![Representation::Pointer],
            CodegenKind::Tuple(_) => vec![Representation::Pointer, Representation::Pointer],
            CodegenKind::Enum(_) => vec![Representation::Pointer],
            CodegenKind::Vec(_) => vec![Representation::Pointer, Representation::Pointer],
            CodegenKind::Unknown => vec![Representation::Pointer],
            CodegenKind::String => vec![Representation::Pointer, Representation::Pointer],
            CodegenKind::Number => vec![Representation::Float],
            CodegenKind::Bool => vec![Representation::Byte],
            CodegenKind::Void => vec![],
        }
    }

    pub fn from_tuple(codegen_kinds: &[CodegenKind]) -> TypeLayout {
        let mut representation = Vec::new();
        for kind in codegen_kinds {
            representation.extend(TypeLayout::represent(kind));
        }

        TypeLayout { representation }
    }

    pub fn new(codegen_kind: &CodegenKind) -> TypeLayout {
        TypeLayout {
            representation: TypeLayout::represent(codegen_kind),
        }
    }

    pub fn size_of(&self, representation: &Representation) -> u32 {
        match representation {
            Representation::Byte => 1,
            Representation::Float => 8,
            Representation::Pointer => 4,
        }
    }

    pub fn width(&self) -> u32 {
        self.representation.len() as u32
    }

    pub fn needs_spill(&self) -> bool {
        if MULTI_VALUE_SUPPORT {
            return false;
        }

        self.width() > 1
    }

    /// Returns the size with an alignment of 4 bytes
    pub fn size(&self) -> u32 {
        align_to_usize(self.representation.iter().map(|v| self.size_of(v)).sum())
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &Representation)> {
        let mut offset = 0;
        self.representation.iter().map(move |rep| {
            let size = self.size_of(rep);
            let result = (offset, rep);
            offset += align_to_usize(size);
            result
        })
    }
}

fn align_to_usize(source: u32) -> u32 {
    (source + 3) / 4 * 4
}
