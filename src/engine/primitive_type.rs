use crate::engine::g::EMPTY_STR;
use cbindgen::ir::{IntKind, PrimitiveType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RPrimitiveType {
    Void,
    Bool,
    Char,
    SChar,
    UChar,
    Char32,
    Float,
    Double,
    VaList,
    PtrDiffT,
    Integer {
        zeroable: bool,
        signed: bool,
        kind: IntKind,
    },
}

impl From<&cbindgen::ir::PrimitiveType> for RPrimitiveType {
    fn from(value: &PrimitiveType) -> Self {
        match value {
            PrimitiveType::Void => RPrimitiveType::Void,
            PrimitiveType::Bool => RPrimitiveType::Bool,
            PrimitiveType::Char => RPrimitiveType::Char,
            PrimitiveType::SChar => RPrimitiveType::SChar,
            PrimitiveType::UChar => RPrimitiveType::UChar,
            PrimitiveType::Char32 => RPrimitiveType::Char32,
            PrimitiveType::Float => RPrimitiveType::Float,
            PrimitiveType::Double => RPrimitiveType::Double,
            PrimitiveType::VaList => RPrimitiveType::VaList,
            PrimitiveType::PtrDiffT => RPrimitiveType::PtrDiffT,
            PrimitiveType::Integer {
                zeroable,
                signed,
                kind,
            } => RPrimitiveType::Integer {
                zeroable: false,
                signed: false,
                kind: IntKind::Short,
            },
        }
    }
}

impl RPrimitiveType {
    pub fn get_ts_type_name(&self) -> &str {
        match self {
            RPrimitiveType::Void => "void",
            RPrimitiveType::Bool => "boolean",
            RPrimitiveType::Char => "string",
            RPrimitiveType::SChar => "string",
            RPrimitiveType::UChar => "string",
            RPrimitiveType::Char32 => "number",
            RPrimitiveType::Float => "number",
            RPrimitiveType::Double => "number",
            RPrimitiveType::VaList => "xxxx",
            RPrimitiveType::PtrDiffT => "xxxx",
            RPrimitiveType::Integer { .. } => "number",
        }
    }

    pub fn get_cpp_type_name(&self) -> &str {
        match self {
            RPrimitiveType::Void => "x",
            RPrimitiveType::Bool => "bool",
            RPrimitiveType::Char => "std::string",
            RPrimitiveType::SChar => "std::string",
            RPrimitiveType::UChar => "std::string",
            RPrimitiveType::Char32 => "int32_t",
            RPrimitiveType::Float => "float_t",
            RPrimitiveType::Double => "double",
            RPrimitiveType::VaList => "xxxx",
            RPrimitiveType::PtrDiffT => "xxxx",
            RPrimitiveType::Integer { .. } => "int32_t",
        }
    }

    pub fn get_ts_type_default_name(&self) -> &str {
        match self {
            RPrimitiveType::Void => EMPTY_STR,
            RPrimitiveType::Bool => "false",
            RPrimitiveType::Char => EMPTY_STR,
            RPrimitiveType::SChar => EMPTY_STR,
            RPrimitiveType::UChar => EMPTY_STR,
            RPrimitiveType::Char32 => EMPTY_STR,
            RPrimitiveType::Float => "-1",
            RPrimitiveType::Double => "-1",
            RPrimitiveType::VaList => EMPTY_STR,
            RPrimitiveType::PtrDiffT => EMPTY_STR,
            RPrimitiveType::Integer { .. } => "-1",
        }
    }
}
