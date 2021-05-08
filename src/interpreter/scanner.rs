use super::{token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    pub source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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
        self.add_token(match ch {
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
                eprintln!("Something went very wrong");
                TokenType::Eof
            }
        });
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
}
