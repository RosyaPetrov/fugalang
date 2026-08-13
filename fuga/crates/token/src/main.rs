#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // sp token
    Eof,
    NewLine,
    Whitespace {
        pub chars: Vec<Pass>
    },
    Comment {
        pub literal: String,
        pub multiline: bool
    },

    Identifier {
        pub literal: String,
    },

    // literals
    Float {
        pub literal: f64,
    },
    Int {
        pub literal: i64,
    },
    String {
        pub literal: String,
    },
    RawString {
        pub literal: String,
    },
    Chat {
        pub literal: char,
    },

    // key word
    Module,
    Use,

    Pub,

    Fn,
    Impl,

    Let,
    Mut,
    Const,

    If,
    Else,

    Switch,
    Select,
    Case,

    Enum,
    Match,

    For,

    Defer,

    Unsafe,

    // operators
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    Caret,          // ^ Power and BitXor

    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=

    ShortDeclare,   // :=
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModuloAssign,   // %=
    PowerAssign,    // ^=

    LogicalAnd,     // &&
    LogicalOr,      // ||

    Ampersand,      // & Ref and BitAnd

    // BitAnd       // & 
    BitOr,          // |
    BitNot,         // ~

    LeftShift,      // <<
    RightShift,     // >>


    Arrow,          // ->
    FatArrow,       // =>
    Range,          // ..
    Variadic,       // ...

    Directive,      // #

    // punctuation
    LeftParen,      // (
    RightParen,     // )

    LeftBrace,      // {
    RightBrace,     // }

    LeftBracket,    // [
    RightBracket,   // ]

    Bang,           // !
    Question,       // ?

    Comma,          // ,
    Dot,            // .
    Colon,          // :
    PathSeparator,  // ::
    Semicolon,      // ;
}

pub enum Pass {
    Space,
    Tabulation
}

pub struct Position {
    pub Colomn: u64,
    pub Line: u64,
}

pub struct Token {
    pub Type: Type,
    pub Pos: Position,

    pub StartOffset: u64,
    pub EndOffset: u64,
}