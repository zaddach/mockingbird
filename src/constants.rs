use windows::Win32::System::WinRT::Metadata::{ELEMENT_TYPE_BOOLEAN, ELEMENT_TYPE_CHAR, ELEMENT_TYPE_CLASS, ELEMENT_TYPE_I, ELEMENT_TYPE_I1, ELEMENT_TYPE_I2, ELEMENT_TYPE_I4, ELEMENT_TYPE_I8, ELEMENT_TYPE_PTR, ELEMENT_TYPE_R4, ELEMENT_TYPE_R8, ELEMENT_TYPE_U, ELEMENT_TYPE_U1, ELEMENT_TYPE_U2, ELEMENT_TYPE_U4, ELEMENT_TYPE_U8, ELEMENT_TYPE_VOID};
use windows_metadata::Type;


/// Constant array mapping Type enum variants to their names and indices
/// Each tuple contains (type_name, type_index) where type_index is just an incremental counter
pub const TYPE_VARIANTS: &[(& str, u8)] = &[
    ("Void", ELEMENT_TYPE_VOID.0),
    ("Bool", ELEMENT_TYPE_BOOLEAN.0),
    ("I8", ELEMENT_TYPE_I1.0),
    ("U8", ELEMENT_TYPE_U1.0),
    ("I16", ELEMENT_TYPE_I2.0),
    ("U16", ELEMENT_TYPE_U2.0),
    ("I32", ELEMENT_TYPE_I4.0),
    ("U32", ELEMENT_TYPE_U4.0),
    ("I64", ELEMENT_TYPE_I8.0),
    ("U64", ELEMENT_TYPE_U8.0),
    ("F32", ELEMENT_TYPE_R4.0),
    ("F64", ELEMENT_TYPE_R8.0),
    ("Char", ELEMENT_TYPE_CHAR.0),
    ("ISize", ELEMENT_TYPE_I.0),
    ("USize", ELEMENT_TYPE_U.0),
    ("Name", ELEMENT_TYPE_CLASS.0),        // Type::Name(Name)
    ("PtrMut", ELEMENT_TYPE_PTR.0),      // Type::PtrMut(Box<Type>, usize)
    ("PtrConst", 0x80),    // Type::PtrConst(Box<Type>, usize)
    ("ConstRef", 0x81),    // Type::ConstRef(Box<Type>)
];

/// Maps a Type enum variant to its corresponding index in TYPE_VARIANTS
pub fn get_type_variant_index(type_variant: &Type) -> u8 {
    match type_variant {
        Type::Void => ELEMENT_TYPE_VOID.0,
        Type::Bool => ELEMENT_TYPE_BOOLEAN.0,
        Type::I8 => ELEMENT_TYPE_I1.0,
        Type::U8 => ELEMENT_TYPE_U1.0,
        Type::I16 => ELEMENT_TYPE_I2.0,
        Type::U16 => ELEMENT_TYPE_U2.0,
        Type::I32 => ELEMENT_TYPE_I4.0,
        Type::U32 => ELEMENT_TYPE_U4.0,
        Type::I64 => ELEMENT_TYPE_I8.0,
        Type::U64 => ELEMENT_TYPE_U8.0,
        Type::F32 => ELEMENT_TYPE_R4.0,
        Type::F64 => ELEMENT_TYPE_R8.0,
        Type::Char => ELEMENT_TYPE_CHAR.0,
        Type::ISize => ELEMENT_TYPE_I.0,
        Type::USize => ELEMENT_TYPE_U.0,
        Type::Name(_) => ELEMENT_TYPE_CLASS.0,
        Type::PtrMut(_, _) => ELEMENT_TYPE_PTR.0,
        Type::PtrConst(_, _) => 0x80,
        Type::ConstRef(_) => 0x81,
        // Handle any potential future variants with a default case
        _ => panic!("Unknown Type variant: {type_variant:?}"),
    }
}

