use windows_metadata::Type;

/// Constant array mapping Type enum variants to their names and indices
/// Each tuple contains (type_name, type_index) where type_index is just an incremental counter
pub const TYPE_VARIANTS: &[(& str, usize)] = &[
    ("Void", 0),
    ("Bool", 1),
    ("I8", 2),
    ("U8", 3),
    ("I16", 4),
    ("U16", 5),
    ("I32", 6),
    ("U32", 7),
    ("I64", 8),
    ("U64", 9),
    ("F32", 10),
    ("F64", 11),
    ("Char", 12),
    ("ISize", 13),
    ("USize", 14),
    ("Name", 15),        // Type::Name(Name)
    ("PtrMut", 16),      // Type::PtrMut(Box<Type>, usize)
    ("PtrConst", 17),    // Type::PtrConst(Box<Type>, usize)
    ("ConstRef", 18),    // Type::ConstRef(Box<Type>)
];

/// Maps a Type enum variant to its corresponding index in TYPE_VARIANTS
pub fn get_type_variant_index(type_variant: &Type) -> usize {
    match type_variant {
        Type::Void => 0,
        Type::Bool => 1,
        Type::I8 => 2,
        Type::U8 => 3,
        Type::I16 => 4,
        Type::U16 => 5,
        Type::I32 => 6,
        Type::U32 => 7,
        Type::I64 => 8,
        Type::U64 => 9,
        Type::F32 => 10,
        Type::F64 => 11,
        Type::Char => 12,
        Type::ISize => 13,
        Type::USize => 14,
        Type::Name(_) => 15,
        Type::PtrMut(_, _) => 16,
        Type::PtrConst(_, _) => 17,
        Type::ConstRef(_) => 18,
        // Handle any potential future variants with a default case
        _ => panic!("Unknown Type variant: {type_variant:?}"),
    }
}

