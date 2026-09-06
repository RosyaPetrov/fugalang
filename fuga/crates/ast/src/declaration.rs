use crate::{
    expression::Expr,
    modifiers::Modifiers,
    path::Path,
    pattern::Pattern,
    statement::Block,
    types::{Generics, Type},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Module { name: Path },
    Package { name: Path, modifiers: Modifiers },
    Import { path: Path, alias: Option<String> },
    Goto { tag: String },
    Variable(Variable),
    Type(TypeDeclaration),
    Struct(StructDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Impl(ImplDeclaration),
    Function(Function),
    Unsafe(Block),
    Directive(Directive),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub value: Option<Expr>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    pub name: String,
    pub generics: Generics,
    pub ty: Type,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: Option<String>,
    pub ty: Type,
    pub default: Option<Expr>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub name: String,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDeclaration {
    pub name: String,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: String,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplDeclaration {
    pub target: Path,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub body: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub generics: Generics,
    pub params: Vec<Parameter>,
    pub return_types: Vec<Type>,
    pub body: Block,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: Path,
    pub args: Vec<DirectiveArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveArg {
    pub name: Option<Path>,
    pub value: Expr,
}
