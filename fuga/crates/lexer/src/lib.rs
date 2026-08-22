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
            '\n' => { self.pos.line += 1; self.pos.column = 1; TokenType::NewLine },

            ' ' | '\t' => {
                let mut chars = vec![match ch {
                    ' ' => Whitespace::Space,
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

            '"' => { return Token::new(self.lex_string(ch), Span { start: start, end: self.pos }) }
            _ if ch.is_ascii_digit() => { return  Token::new(self.lex_number(ch) , Span { start: start, end: self.pos }) }

            '+' => self.match_next('=', TokenType::PlusAssign, TokenType::Plus),
            '-' => match self.peek() { Some('=') => { self.nextch(); TokenType::MinusAssign }
                Some('>') => { self.nextch(); TokenType::Arrow }
                _ => TokenType::Minus 
            }
            '*' => self.match_next('=', TokenType::MultiplyAssign, TokenType::Multiply),
            '/' => self.match_next('=', TokenType::DivideAssign, TokenType::Divide),
            '%' => self.match_next('=', TokenType::ModuloAssign, TokenType::Modulo),
            '^' => self.match_next('=', TokenType::PowerAssign, TokenType::Caret),

            '=' => match self.peek() { Some('=') => { self.nextch(); TokenType::Equal }
                Some('>') => { self.nextch(); TokenType::FatArrow }
                _ => TokenType::Assign, 
            }

            '!' => self.match_next('=', TokenType::NotEqual, TokenType::Bang),

            '<' => match self.peek() { Some('=') => { self.nextch(); TokenType::LessEqual }
                Some('<') => { self.nextch(); TokenType::LeftShift }
                _ => TokenType::Less, 
            }

            '>' => match self.peek() {  Some('=') => { self.nextch(); TokenType::GreaterEqual }
                Some('>') => { self.nextch(); TokenType::RightShift }
                _ => TokenType::Greater,
            }

            ':' => match self.peek() { Some('=') => { self.nextch(); TokenType::ShortDeclare }
                Some(':') => { self.nextch(); TokenType::PathSeparator }
                _ => TokenType::Colon,
            }

            '&' => self.match_next('&', TokenType::LogicalAnd, TokenType::Ampersand),
            '|' => self.match_next('|', TokenType::LogicalOr, TokenType::BitOr),
            '~' => TokenType::BitNot,
            '.' => match self.peek() { Some('.') => { self.nextch();
                    if self.peek() == Some('.') { self.nextch(); TokenType::Variadic } else { TokenType::Range } }
                _ => TokenType::Dot,
            }
        
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
            _ => TokenType::Illegal { literal: ch.to_string() }
        };

        Token::new( token_type, Span { start, end: self.pos, } )
    }

    fn lex_word(&mut self, first: char) -> TokenType {
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
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,

            _ => TokenType::Identifier {
                literal: value,
            },
        }
    }

    fn lex_number(&mut self, first: char) -> TokenType {
        let mut value = String::new();
        value.push(first);

        // main 
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.nextch().unwrap());
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            let mut chars = self.chars.clone();

            // skip first dot
            chars.next();

            match chars.next().map(|(_, ch)| ch) {
                // 1.23
                Some(ch) if ch.is_ascii_digit() => {
                    value.push(self.nextch().unwrap());

                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            value.push(self.nextch().unwrap());
                        } else {
                            break;
                        }
                    }

                    // 1.23i
                    if self.peek() == Some('i') {
                        value.push(self.nextch().unwrap());

                        return TokenType::Complex {
                            literal: value,
                        };
                    }

                    
                    return TokenType::Float {
                        literal: value.parse::<f64>().unwrap(),
                    };
                }

                // 1..10
                Some('.') => {
                    // Range / Variadic
                }

                // 1.abc
                _ => {
                    // next use fn Lexer.next_token return Dot token
                }
            }
        }

        // 123i
        if self.peek() == Some('i') {
            value.push(self.nextch().unwrap());

            return TokenType::Complex {
                literal: value,
            };
        }

        TokenType::Integer {
            literal: value.parse::<i64>().unwrap(),
        }
    }

    fn lex_string(&mut self, first: char) -> TokenType {
        if first == '"' {
            let mut value = String::new();

            loop {
                let r = self.peek();
                match r {
                    Some('"') => {
                        self.nextch();
                        return TokenType::String { literal: value };
                    }

                    Some('\\') => {
                        // Todo: n, t
                    }

                    Some(ch) => {
                        value.push(ch);
                        self.nextch();
                    }

                    None => {
                        // Todo error;
                        panic!("Unterminated string");

                    }
                }
            }
        } else {
            panic!("11")
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|(_, ch)| ch)
    }

    fn nextch(&mut self) -> Option<char> {
        let (_, ch) = self.chars.next()?;
        self.pos.offset += ch.len_utf8();
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