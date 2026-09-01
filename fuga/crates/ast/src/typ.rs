use crate::pattern;

#[derive(Debug, Clone, PartialEq)]
pub enum Typ {
    Unit,

    Bool(bool),
    Char(char),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),

    F32(f32),
    F64(f64),

    C32 {
        real: f32,
        imaginary: f32,
    },
    C64 {
        real: f64,
        imaginary: f64,
    },

    Str(String),

    // Struct[X, Y, Z]
    // alice type
    Named {
        name: String,
        generics: Option<Vec<Typ>>,
    },

    Generics(pattern::Pattern), // Tuple 1 [T, Y, E]
}
