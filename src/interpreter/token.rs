use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: &'a str,
    line: i32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: &'a str, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
