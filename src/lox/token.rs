use super::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub string_literal: Option<String>,
    pub number_literal: Option<f64>,
    pub line: i32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        string_literal: Option<String>,
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
