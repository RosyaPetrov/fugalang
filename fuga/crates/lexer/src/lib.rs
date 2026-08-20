use token::{Position, Span, Token, TokenType, Whitespace};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::CharIndices<'a>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
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
            '/' => self.match_next('=', TokenType::DivideAssign, TokenType::Divide),
            '%' => self.match_next('=', TokenType::ModuloAssign, TokenType::Modulo),
            '^' => self.match_next('=', TokenType::PowerAssign, TokenType::Caret),

            '=' => match self.peek() { Some('=') => { self.nextch(); TokenType::Equal }
                Some('>') => { self.nextch(); TokenType::FatArrow }
                _ => TokenType::Assign, },

            '!' => self.match_next('=', TokenType::NotEqual, TokenType::Bang),

            '<' => match self.peek() { Some('=') => { self.nextch(); TokenType::LessEqual }
                Some('<') => { self.nextch(); TokenType::LeftShift }
                _ => TokenType::Less, },

            '>' => match self.peek() {  Some('=') => { self.nextch(); TokenType::GreaterEqual }
                Some('>') => { self.nextch(); TokenType::RightShift }
                _ => TokenType::Greater,
            },

            ':' => match self.peek() { Some('=') => { self.nextch(); TokenType::ShortDeclare }
                Some(':') => { self.nextch(); TokenType::PathSeparator }
                _ => TokenType::Colon,
            },

            '&' => self.match_next('&', TokenType::LogicalAnd, TokenType::Ampersand),
            '|' => self.match_next('|', TokenType::LogicalOr, TokenType::BitOr),
            '~' => TokenType::BitNot,
            '.' => match self.peek() { Some('.') => { self.nextch();
                    if self.peek() == Some('.') { self.nextch(); TokenType::Variadic } else { TokenType::Range } }
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
