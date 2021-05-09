use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    string_literal: Option<&'a str>,
    number_literal: Option<f64>,
    line: i32,
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
