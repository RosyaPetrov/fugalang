use crate::{expr::Expr, pattern::Pattern, stmt::Stmt, typ::Typ};

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    // mod name
    Module {
        name: Box<Expr>, // name::name::name.name
    },

    // pkg main
    Pkg {
        name: Box<Expr>, // name::name::name.name
        is_public: bool,
    },

    // imp ("", "")
    Imports(Vec<Decl>), // Import

    Import {
        imports: Box<Pattern>, // name
        alias: Option<String>,
    },

    // let x: type = expr
    // x := expr
    Var {
        name: Box<Pattern>, // Tuple 1
        mutable: bool,
        ty: Option<Typ>,
        // if value is None and mutable is false = can be changed once.
        // ! if value is None and is_comptime is true = error build
        value: Option<Expr>,

        is_comptime: bool, // const
        is_public: bool,   // pub
    },

    // type Name struct | interface | type
    Type {
        structure: Option<Box<Decl>>,
        interface: Option<Box<Decl>>,
        typ: Option<Box<Typ>>,
    },

    Struct {
        name: String,
        generics: Option<Typ>, // Generics 1 [T, Y, E]
        fields: Box<Pattern>,

        is_public: bool,
        is_comptime: bool,
    },

    Interface {
        name: String,
        generics: Option<Typ>, // Generics 1 [T, Y, E]
        fields: Box<Pattern>,

        is_public: bool,
        is_comptime: bool,
    },

    Enum {
        name: String,
        fields: Box<Pattern>, // Tuple { field1: type, field2: type, field3: type } (type, type)

        is_public: bool,   // pub
        is_comptime: bool, // const
    },

    Impl {
        name: Box<Expr>,       // name::name::name.name
        fields: Box<Pattern>, // Tuple { field1: type := expr, field2: type := expr, field3: type := expr}
        generics: Option<Typ>, // Generics 1 [T, Y, E]
        body: Vec<Decl>,      // Vec<Decl> (Decl, Decl, Decl)
    },

    Function {
        name: String,
        generics: Option<Typ>,              // Generics 1 [T, Y, E]
        params: Box<Pattern>,               // Tuple (arg1: T, arg2: U)
        return_types: Option<Box<Pattern>>, // Tuple (T, U) (name: type, name: type)
        body: Vec<Stmt>,                    // Vec<Stmt> (Stmt, Stmt, Stmt)

        is_public: bool,   // pub
        is_macros: bool,   // macros
        is_comptime: bool, // opz
    },

    Unsafe {
        body: Vec<Box<Stmt>>,
    },

    Directive {
        derectives: Vec<Decl>, // DirectiveArg
    },

    DirectiveArg {
        name: Box<Expr>,
        arg: Box<Pattern>, // Tuple
    },
}
