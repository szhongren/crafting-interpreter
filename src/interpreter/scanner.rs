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
            '(' => Option::from(TokenType::LeftParen),
            ')' => Option::from(TokenType::RightParen),
            '{' => Option::from(TokenType::LeftBrace),
            '}' => Option::from(TokenType::RightBrace),
            ',' => Option::from(TokenType::Comma),
            '.' => Option::from(TokenType::Dot),
            '-' => Option::from(TokenType::Minus),
            '+' => Option::from(TokenType::Plus),
            ';' => Option::from(TokenType::Semicolon),
            '*' => Option::from(TokenType::Star),
            '!' => Option::from(if self.match_char('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => Option::from(if self.match_char('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => Option::from(if self.match_char('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => Option::from(if self.match_char('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Option::None
                } else {
                    Option::from(TokenType::Slash)
                }
            }
            // ignore whitespace
            ' ' | '\r' | '\t' => Option::None,
            '\n' => {
                self.line += 1;
                Option::None
            }
            ch => {
                self.error(format!("unexpected character: {}", ch).as_str());
                Option::None
            }
        };
        if token.is_some() {
            self.add_token(token.unwrap());
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.get_current_char();
        self.current += 1;
        ch
    }

    fn get_current_char(&mut self) -> char {
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .expect("self.current is greater than the number of chars in self.source");
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

    fn match_char(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_current_char() != ch {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_current_char()
        }
    }
}
