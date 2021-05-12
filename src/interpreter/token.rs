use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub string_literal: Option<&'a str>,
    pub number_literal: Option<f64>,
    pub line: i32,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        string_literal: Option<&'a str>,
        number_literal: Option<f64>,
        line: i32,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            string_literal,
            number_literal,
            line,
        }
    }
}
