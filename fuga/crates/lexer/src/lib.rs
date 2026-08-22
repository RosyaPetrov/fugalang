use token::{Position, Span, Token, TokenType, Whitespace};

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: std::str::CharIndices<'a>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices(),
            pos: Position {
                column: 1,
                line: 1,
                offset: 0,
            },
        }
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.pos;

        let Some(ch) = self.nextch() else {
            return Token::new(
                TokenType::Eof,
                Span {
                    start: start,
                    end: self.pos,
                },
            );
        };

        let token_type = match ch {
            '\n' => TokenType::NewLine,

            ' ' | '\t' | '\r' => {
                let mut chars = vec![match ch {
                    ' ' => Whitespace::Space,
                    '\r' => Whitespace::Space,
                    '\t' => Whitespace::Tab,
                    _ => unreachable!(),
                }];

                loop {
                    match self.peek() {
                        Some(' ') => {
                            self.nextch();
                            chars.push(Whitespace::Space);
                        }

                        Some('\t') => {
                            self.nextch();
                            chars.push(Whitespace::Tab);
                        }

                        _ => break,
                    }
                }

                TokenType::Whitespace { chars }
            }

            '/' => match self.peek() {
                Some('/') | Some('*') => self.lex_comment(),

                Some('=') => {
                    self.nextch();
                    TokenType::DivideAssign
                }

                _ => TokenType::Divide,
            },

            _ if is_word_char(ch) => self.lex_identifier(ch),
            _ if ch.is_ascii_digit() => self.lex_number(ch),

            '"' => self.lex_string(),
            '`' => self.lex_raw_string(),
            '\'' => self.lex_char(ch),

            '+' => self.match_next('=', TokenType::PlusAssign, TokenType::Plus),
            '-' => match self.peek() {
                Some('=') => {
                    self.nextch();
                    TokenType::MinusAssign
                }
                Some('>') => {
                    self.nextch();
                    TokenType::Arrow
                }
                _ => TokenType::Minus,
            },
            '*' => self.match_next('=', TokenType::MultiplyAssign, TokenType::Multiply),
            '%' => self.match_next('=', TokenType::ModuloAssign, TokenType::Modulo),
            '^' => self.match_next('=', TokenType::PowerAssign, TokenType::Caret),

            '=' => match self.peek() {
                Some('=') => {
                    self.nextch();
                    TokenType::Equal
                }
                Some('>') => {
                    self.nextch();
                    TokenType::FatArrow
                }
                _ => TokenType::Assign,
            },

            '!' => self.match_next('=', TokenType::NotEqual, TokenType::Bang),

            '<' => match self.peek() {
                Some('=') => {
                    self.nextch();
                    TokenType::LessEqual
                }
                Some('<') => {
                    self.nextch();
                    TokenType::LeftShift
                }
                _ => TokenType::Less,
            },

            '>' => match self.peek() {
                Some('=') => {
                    self.nextch();
                    TokenType::GreaterEqual
                }
                Some('>') => {
                    self.nextch();
                    TokenType::RightShift
                }
                _ => TokenType::Greater,
            },

            ':' => match self.peek() {
                Some('=') => {
                    self.nextch();
                    TokenType::ShortDeclare
                }
                Some(':') => {
                    self.nextch();
                    TokenType::PathSeparator
                }
                _ => TokenType::Colon,
            },

            '&' => self.match_next('&', TokenType::LogicalAnd, TokenType::Ampersand),
            '|' => self.match_next('|', TokenType::LogicalOr, TokenType::BitOr),
            '~' => TokenType::BitNot,
            '.' => match self.peek() {
                Some('.') => {
                    self.nextch();
                    if self.peek() == Some('.') {
                        self.nextch();
                        TokenType::Variadic
                    } else {
                        TokenType::Range
                    }
                }
                _ => TokenType::Dot,
            },

            '#' => TokenType::Directive,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '?' => TokenType::Question,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            _ => TokenType::Illegal {
                literal: ch.to_string(),
            },
        };

        Token::new(
            token_type,
            Span {
                start,
                end: self.pos,
            },
        )
    }

    fn lex_identifier(&mut self, first: char) -> TokenType {
        let mut value = String::new();
        value.push(first);

        while let Some(ch) = self.peek() {
            if is_word_char(ch) {
                value.push(self.nextch().unwrap());
            } else {
                break;
            }
        }

        match value.as_str() {
            "module" => TokenType::Module,
            "use" => TokenType::Use,
            "pub" => TokenType::Pub,
            "priv" => TokenType::Priv,
            "let" => TokenType::Let,
            "mut" => TokenType::Mut,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "struct" => TokenType::Struct,
            "type" => TokenType::Type,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "select" => TokenType::Select,
            "enum" => TokenType::Enum,
            "match" => TokenType::Match,
            "for" => TokenType::For,
            "defer" => TokenType::Defer,
            "unsafe" => TokenType::Unsafe,

            _ => TokenType::Identifier { literal: value },
        }
    }

    fn lex_number(&mut self, first: char) -> TokenType {
        let mut value = String::new();
        value.push(first);

        // part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.nextch().unwrap());
            } else {
                break;
            }
        }

        // float
        if self.peek() == Some('.') {
            let mut chars = self.chars.clone();
            chars.next();

            if chars.next().map(|(_, ch)| ch) != Some('.') {
                value.push(self.nextch().unwrap());

                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        value.push(self.nextch().unwrap());
                    } else {
                        break;
                    }
                }

                if self.peek() == Some('i') {
                    value.push(self.nextch().unwrap());

                    return TokenType::Complex { literal: value };
                }

                return TokenType::Float { literal: value };
            }
        }

        // complex integer: 123i
        if self.peek() == Some('i') {
            value.push(self.nextch().unwrap());

            return TokenType::Complex { literal: value };
        }

        TokenType::Integer { literal: value }
    }

    fn lex_string(&mut self) -> TokenType {
        let mut value = String::new();

        loop {
            match self.peek() {
                Some('\\') => {
                    self.nextch();

                    let ch = match self.nextch() {
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some('\\') => '\\',
                        Some('\'') => '\'',
                        Some('"') => '"',
                        Some(ch) => panic!("Unknown escape sequence: \\{}", ch),
                        None => panic!("Unterminated string"),
                    };

                    value.push(ch);
                }

                Some('"') => {
                    self.nextch();
                    return TokenType::String { literal: value };
                }

                Some('\n') => {
                    panic!("Unterminated string")
                }

                Some(ch) => {
                    value.push(ch);
                    self.nextch();
                }

                None => {
                    panic!("Unterminated string");
                }
            }
        }
    }

    fn lex_raw_string(&mut self) -> TokenType {
        let mut value = String::new();

        loop {
            let r = self.peek();
            match r {
                Some('`') => {
                    self.nextch();
                    return TokenType::RawString { literal: value };
                }

                Some(ch) => {
                    value.push(ch);
                    self.nextch();
                }

                None => {
                    // Todo error;
                    panic!("Unterminated raw string");
                }
            }
        }
    }

    fn lex_char(&mut self, first: char) -> TokenType {
        if first != '\'' {
            panic!("Expected char literal");
        }

        let ch = match self.nextch() {
            Some('\\') => match self.nextch() {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some('"') => '"',
                Some(ch) => panic!("Unknown escape sequence: \\{}", ch),
                None => panic!("Unterminated char literal"),
            },

            Some('\'') => {
                // ''
                panic!("Empty char literal");
            }

            Some(ch) => ch,

            None => {
                panic!("Unterminated char literal");
            }
        };

        match self.nextch() {
            Some('\'') => TokenType::Char { literal: ch },

            Some(ch) => panic!("Expected closing ', found '{}'", ch),

            None => panic!("Unterminated char literal"),
        }
    }

    fn lex_comment(&mut self) -> TokenType {
        match self.peek() {
            // // comment
            Some('/') => {
                self.nextch();

                let mut literal = String::new();

                while let Some(ch) = self.peek() {
                    if ch == '\n' {
                        break;
                    }

                    literal.push(self.nextch().unwrap());
                }

                TokenType::Comment {
                    literal,
                    multiline: false,
                }
            }

            // /* comment */
            Some('*') => {
                self.nextch();

                let mut literal = String::new();

                loop {
                    let Some(ch) = self.nextch() else {
                        panic!("unterminated comment")
                    };

                    if ch == '*' && self.peek() == Some('/') {
                        self.nextch();
                        break;
                    }

                    literal.push(ch);
                }

                TokenType::Comment {
                    literal,
                    multiline: true,
                }
            }

            _ => TokenType::Divide,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|(_, ch)| ch)
    }

    fn nextch(&mut self) -> Option<char> {
        let (_, ch) = self.chars.next()?;

        self.pos.offset += ch.len_utf8();

        if ch == '\n' {
            self.pos.line += 1;
            self.pos.column = 1;
        } else {
            self.pos.column += 1;
        }

        Some(ch)
    }

    fn match_next(
        &mut self,
        expected: char,
        matched: TokenType,
        unmatched: TokenType,
    ) -> TokenType {
        if self.peek() == Some(expected) {
            self.nextch();
            matched
        } else {
            unmatched
        }
    }
}

fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
