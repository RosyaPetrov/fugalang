#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Eof,
    NewLine,                                      // \n
    Whitespace { chars: Vec<Whitespace> },        //
    Comment { literal: String, multiline: bool }, // // comment

    Identifier { literal: String }, // var

    Float { literal: String },     // 1.4
    Integer { literal: String },   // 123
    Complex { literal: String },   // 1.3i
    String { literal: String },    // "string"
    RawString { literal: String }, // `string`
    Char { literal: char },        // 'с'

    Module, // module
    Use,    // use
    Pub,    // pub
    Priv,   // priv
    Fn,     // fn
    Return, // return
    Struct, // struct
    Type,   // type
    Impl,   // impl
    Let,    // let
    Mut,    // mut
    Const,  // const
    If,     // if
    Else,   // else
    Switch, // switch
    Select, // select
    Case,   // case
    Enum,   // enum
    Match,  // match
    For,    // for
    Defer,  // defer
    Unsafe, // unsafe

    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Caret,    // ^ Power and BitXor

    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    ShortDeclare,   // :=
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModuloAssign,   // %=
    PowerAssign,    // ^=

    LogicalAnd, // &&
    LogicalOr,  // ||

    Ampersand, // & Ref and BitAnd
    BitOr,     // |
    BitNot,    // ~

    LeftShift,  // <<
    RightShift, // >>

    Arrow,    // ->
    FatArrow, // =>
    Range,    // ..
    Variadic, // ...

    Directive, // #

    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    Bang,     // !
    Question, // ?

    Comma,         // ,
    Dot,           // .
    Colon,         // :
    PathSeparator, // ::
    Semicolon,     // ;

    Illegal { literal: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Whitespace {
    Space,
    Tab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub offset: usize,
    pub column: usize,
    pub line: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }
}
