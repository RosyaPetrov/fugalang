use token::{Position, Token, TokenType, Whitespace};

pub struct Lexer {
    src: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into().chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.current;
        let position = self.position();

        let Some(ch) = self.advance() else {
            return Token::new(TokenType::Eof, position, start, start);
        };

        let token_type = match ch {
            '\n' => TokenType::NewLine,
            ' ' | '\t' => self.read_whitespace(ch),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(ch),
            '0'..='9' => self.read_number(ch),
            '+' => self.match_next('=', TokenType::PlusAssign, TokenType::Plus),
            '-' => self.match_next('=', TokenType::MinusAssign, TokenType::Minus),
            '*' => self.match_next('=', TokenType::MultiplyAssign, TokenType::Multiply),
            '/' => self.match_next('=', TokenType::DivideAssign, TokenType::Divide),
            '%' => self.match_next('=', TokenType::ModuloAssign, TokenType::Modulo),
            '^' => self.match_next('=', TokenType::PowerAssign, TokenType::Caret),
            '=' => self.match_next('=', TokenType::Equal, TokenType::Assign),
            '!' => self.match_next('=', TokenType::NotEqual, TokenType::Bang),
            '<' => {
                if self.consume_if('=') {
                    TokenType::LessEqual
                } else if self.consume_if('<') {
                    TokenType::LeftShift
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.consume_if('=') {
                    TokenType::GreaterEqual
                } else if self.consume_if('>') {
                    TokenType::RightShift
                } else {
                    TokenType::Greater
                }
            }
            ':' => {
                if self.consume_if('=') {
                    TokenType::ShortDeclare
                } else {
                    self.match_next(':', TokenType::PathSeparator, TokenType::Colon)
                }
            }
            '&' => self.match_next('&', TokenType::LogicalAnd, TokenType::Ampersand),
            '|' => self.match_next('|', TokenType::LogicalOr, TokenType::BitOr),
            '~' => TokenType::BitNot,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '?' => TokenType::Question,
            ',' => TokenType::Comma,
            '.' => {
                if self.consume_if('.') {
                    self.match_next('.', TokenType::Variadic, TokenType::Range)
                } else {
                    TokenType::Dot
                }
            }
            ';' => TokenType::Semicolon,
            '#' => TokenType::Directive,
            '"' => self.read_string(),
            '\'' => self.read_char(),
            other => TokenType::Illegal {
                literal: other.to_string(),
            },
        };

        Token::new(token_type, position, start, self.current)
    }

    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.src.get(self.current).copied()?;
        self.current += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.current).copied()
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_next(
        &mut self,
        expected: char,
        matched: TokenType,
        unmatched: TokenType,
    ) -> TokenType {
        if self.consume_if(expected) {
            matched
        } else {
            unmatched
        }
    }

    fn read_whitespace(&mut self, first: char) -> TokenType {
        let mut chars = vec![whitespace_from_char(first)];

        while matches!(self.peek(), Some(' ' | '\t')) {
            let ch = self.advance().expect("peeked whitespace must be readable");
            chars.push(whitespace_from_char(ch));
        }

        TokenType::Whitespace { chars }
    }

    fn read_identifier(&mut self, first: char) -> TokenType {
        let mut literal = String::from(first);

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                literal.push(
                    self.advance()
                        .expect("peeked identifier char must be readable"),
                );
            } else {
                break;
            }
        }

        keyword_or_identifier(literal)
    }

    fn read_number(&mut self, first: char) -> TokenType {
        let mut literal = String::from(first);

        while matches!(self.peek(), Some('0'..='9')) {
            literal.push(self.advance().expect("peeked digit must be readable"));
        }

        if self.peek() == Some('.') && matches!(self.src.get(self.current + 1), Some('0'..='9')) {
            literal.push(self.advance().expect("peeked dot must be readable"));

            while matches!(self.peek(), Some('0'..='9')) {
                literal.push(
                    self.advance()
                        .expect("peeked fractional digit must be readable"),
                );
            }

            return TokenType::Float {
                literal: literal.parse().expect("lexer collected a valid float"),
            };
        }

        TokenType::Int {
            literal: literal.parse().expect("lexer collected a valid int"),
        }
    }

    fn read_string(&mut self) -> TokenType {
        let mut literal = String::new();

        while let Some(ch) = self.advance() {
            match ch {
                '"' => return TokenType::String { literal },
                '\\' => {
                    if let Some(escaped) = self.advance() {
                        literal.push(escaped);
                    }
                }
                other => literal.push(other),
            }
        }

        TokenType::Illegal { literal }
    }

    fn read_char(&mut self) -> TokenType {
        let Some(literal) = self.advance() else {
            return TokenType::Illegal {
                literal: String::new(),
            };
        };

        if self.consume_if('\'') {
            TokenType::Char { literal }
        } else {
            TokenType::Illegal {
                literal: literal.to_string(),
            }
        }
    }
}

fn whitespace_from_char(ch: char) -> Whitespace {
    match ch {
        ' ' => Whitespace::Space,
        '\t' => Whitespace::Tabulation,
        _ => unreachable!("lexer only passes whitespace chars here"),
    }
}

fn keyword_or_identifier(literal: String) -> TokenType {
    match literal.as_str() {
        "module" => TokenType::Module,
        "use" => TokenType::Use,
        "pub" => TokenType::Pub,
        "fn" => TokenType::Fn,
        "impl" => TokenType::Impl,
        "let" => TokenType::Let,
        "mut" => TokenType::Mut,
        "const" => TokenType::Const,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "switch" => TokenType::Switch,
        "select" => TokenType::Select,
        "case" => TokenType::Case,
        "enum" => TokenType::Enum,
        "match" => TokenType::Match,
        "for" => TokenType::For,
        "defer" => TokenType::Defer,
        "unsafe" => TokenType::Unsafe,
        _ => TokenType::Identifier { literal },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_basic_tokens() {
        let mut lexer = Lexer::new("let x := 42");

        assert_eq!(lexer.next_token().token_type, TokenType::Let);
        assert!(matches!(
            lexer.next_token().token_type,
            TokenType::Whitespace { .. }
        ));
        assert_eq!(
            lexer.next_token().token_type,
            TokenType::Identifier {
                literal: "x".to_string()
            }
        );
        assert!(matches!(
            lexer.next_token().token_type,
            TokenType::Whitespace { .. }
        ));
        assert_eq!(lexer.next_token().token_type, TokenType::ShortDeclare);
        assert!(matches!(
            lexer.next_token().token_type,
            TokenType::Whitespace { .. }
        ));
        assert_eq!(
            lexer.next_token().token_type,
            TokenType::Int { literal: 42 }
        );
        assert_eq!(lexer.next_token().token_type, TokenType::Eof);
    }

    #[test]
    fn tracks_line_and_column() {
        let mut lexer = Lexer::new("let\nx");

        assert_eq!(lexer.next_token().position, Position { line: 1, column: 1 });
        assert_eq!(lexer.next_token().position, Position { line: 1, column: 4 });
        assert_eq!(lexer.next_token().position, Position { line: 2, column: 1 });
    }
}
