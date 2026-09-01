use crate::expr::Expr;
use crate::ty::Ty;

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    // pub mut x: type = value || pub const x: type = value || pub let x: type = value
    Variable {
        name: String,
        mutable: bool,
        ty: Option<Ty>,
        // if value is None and mutable is false = can be changed once.
        // ! if value is None and is_comptime is true = error build
        value: Option<Expr>,

        is_comptime: bool, // const
        is_public: bool,   // pub
    },

    /*
    pub enum Type {
        Text(String),
        Number(i64),
        Boolean(bool),
        complex { real: f64, imaginary: f64 }, // ? Field
    }
    */
    Enum {
        name: String,
        fields: Vec<Ty>, // ? Field and Tuple
        is_public: bool, // pub
    },

    // pub fn foo[T: interface1 + interface2 || interface3, U: interface4](arg: T, arg2: U) -> (T, U) {}
    Function {
        name: String,
        generics: Vec<Ty>,     // ? Tuple [T: interface1 + interface2]
        params: Vec<Param>,    // ? Tuple (arg1: T, arg2: U)
        return_types: Vec<Ty>, // ? Tuple
        // body: Vec<Stmt>,
        is_public: bool, // pub
    },

    // impl Foo[] { ... }
    // impl Struct[T] { ... }
    // impl Type { ... }
    Impl {
        name: String,
        generics: Vec<Ty>, // ? Tuple [T: interface1 + interface2]
        decls: Vec<Decl>,
        is_public: bool, // pub
    },
}

// mut x: type = value

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: Option<String>,
    pub ty: Ty,
}
