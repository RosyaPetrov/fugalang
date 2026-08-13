fn main() {
    let mut lexer = lexer::Lexer::new("let x := 42");

    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.token_type == token::TokenType::Eof {
            break;
        }
    }
}
