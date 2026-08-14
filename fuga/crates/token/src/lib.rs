#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Eof,
    NewLine,
    Whitespace { chars: Vec<Whitespace> },
    Comment { literal: String, multiline: bool },

    Identifier { literal: String },

    Float { literal: f64 },
    Int { literal: i64 },
    String { literal: String },
    RawString { literal: String },
    Char { literal: char },

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

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Caret,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    ShortDeclare,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,

    LogicalAnd,
    LogicalOr,

    Ampersand,
    BitOr,
    BitNot,

    LeftShift,
    RightShift,

    Arrow,
    FatArrow,
    Range,
    Variadic,

    Directive,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Bang,
    Question,

    Comma,
    Dot,
    Colon,
    PathSeparator,
    Semicolon,

    Illegal { literal: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Whitespace {
    Space,
    Tabulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub column: usize,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        position: Position,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Self {
            token_type,
            position,
            start_offset,
            end_offset,
        }
    }
}
