use self::token::{token_type::TokenType, Token};

mod token;

pub struct Scanner<'a> {
    pub source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            // self.start = self.current;
            tokens.push(self.scan_token());
            // tokens.push(Token::new(TokenType::EOF, "", "", self.line))
        }
        // tokens.push(Token::new(TokenType::EOF, "", "", self.line));
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Token {
        let ch = self.advance();
        Token::new(
            match ch {
                '(' => TokenType::LEFT_PAREN,
                ')' => TokenType::RIGHT_PAREN,
                '{' => TokenType::LEFT_BRACE,
                '}' => TokenType::RIGHT_BRACE,
                ',' => TokenType::COMMA,
                '.' => TokenType::DOT,
                '-' => TokenType::MINUS,
                '+' => TokenType::PLUS,
                ';' => TokenType::SEMICOLON,
                '*' => TokenType::STAR,
                _ => {
                    eprintln!("Something went very wrong");
                    TokenType::EOF
                }
            },
            &self.source[self.start..self.current],
            "",
            self.line,
        )
    }

    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        ch
    }
}
