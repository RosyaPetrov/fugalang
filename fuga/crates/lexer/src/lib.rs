use token::{Position, Span, Token, TokenType, Whitespace};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::CharIndices<'a>,
    peekchs: std::str::CharIndices<'a>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Self {
            input: input,
            chars: input.char_indices(),
            peekchs: input.char_indices(),
            pos: Position { column: 1, line: 1, offset: 0 }       
        };
        l.peekchs.next().map(|(_, ch)| ch);
        l
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.pos;

        let Some(ch) = self.nextch() else {
            return Token::new(
                TokenType::Eof,
                Span { start: start, end: self.pos }
            );
        };

        match ch {
            '\n' => {
                return Token::new(TokenType::Eof, Span { start: start, end: self.pos });
            }
            
            ' ' | '\t' => {
                let mut chars = Vec::new();

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

                return Token::new(
                    TokenType::Whitespace { chars },
                    Span {
                        start: start,
                        end: self.pos,
                    },
                );
            }

            '+' => {
                self.nextch();
                return Token::new(TokenType::Plus, Span {start: start, end: self.pos});
            }

            _ => {
               return Token::new(TokenType::Illegal { literal: ch.to_string()}, Span {start: start, end: self.pos});
            }
        }
    }


    fn peek(&mut self) -> Option<char> {
        self.peekchs.next().map(|(_, ch)| ch)
    }

    fn nextch(&mut self) -> Option<char> {
        let (_, ch) = self.chars.next()?;
        self.pos.offset += ch.len_utf8();
        Some(ch)
    }
}
