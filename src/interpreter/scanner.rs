use super::{token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: i32,
    pub had_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", "", self.line));
        self.tokens.clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let ch = self.advance();
        let token = match ch {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            _ => {
                self.error("unexpected character");
                TokenType::Eof
            }
        };
        self.add_token(token);
    }

    fn advance(&mut self) -> char {
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .expect("self.current is greater than the number of chars in self.source");
        self.current += 1;
        ch
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_base(token_type, "");
    }

    fn add_token_base(&mut self, token_type: TokenType, literal: &'a str) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("self.start..self.current is not a valid slice of self.source");
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    pub fn error(&mut self, message: &str) {
        self.report("", message);
    }

    fn report(&mut self, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, location, message);
        self.had_error = true;
    }
}
