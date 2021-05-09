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

    // entry point
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "",
            Option::None,
            Option::None,
            self.line,
        ));
        self.tokens.clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    // parser
    fn scan_token(&mut self) {
        let ch = self.advance();
        let token = match ch {
            '(' => self.generate_token_option(TokenType::LeftParen),
            ')' => self.generate_token_option(TokenType::RightParen),
            '{' => self.generate_token_option(TokenType::LeftBrace),
            '}' => self.generate_token_option(TokenType::RightBrace),
            ',' => self.generate_token_option(TokenType::Comma),
            '.' => self.generate_token_option(TokenType::Dot),
            '-' => self.generate_token_option(TokenType::Minus),
            '+' => self.generate_token_option(TokenType::Plus),
            ';' => self.generate_token_option(TokenType::Semicolon),
            '*' => self.generate_token_option(TokenType::Star),
            '!' => Option::from(if self.match_char('=') {
                self.generate_new_token(TokenType::BangEqual)
            } else {
                self.generate_new_token(TokenType::Bang)
            }),
            '=' => Option::from(if self.match_char('=') {
                self.generate_new_token(TokenType::EqualEqual)
            } else {
                self.generate_new_token(TokenType::Equal)
            }),
            '<' => Option::from(if self.match_char('=') {
                self.generate_new_token(TokenType::LessEqual)
            } else {
                self.generate_new_token(TokenType::Less)
            }),
            '>' => Option::from(if self.match_char('=') {
                self.generate_new_token(TokenType::GreaterEqual)
            } else {
                self.generate_new_token(TokenType::Greater)
            }),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Option::None
                } else {
                    self.generate_token_option(TokenType::Slash)
                }
            }
            // ignore whitespace
            ' ' | '\r' | '\t' => Option::None,
            '\n' => {
                self.line += 1;
                Option::None
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            ch => {
                self.error(format!("unexpected character: {}", ch).as_str());
                Option::None
            }
        };
        if token.is_some() {
            self.add_token(token.unwrap());
        }
    }

    fn generate_token_option(&mut self, token_type: TokenType) -> Option<Token<'a>> {
        Option::from(self.generate_new_token(token_type))
    }

    fn generate_new_token(&mut self, token_type: TokenType) -> Token<'a> {
        Token::new(
            token_type,
            self.get_lexeme(),
            Option::None,
            Option::None,
            self.line,
        )
    }

    // errors
    pub fn error(&mut self, message: &str) {
        self.report("", message);
    }

    fn report(&mut self, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, location, message);
        self.had_error = true;
    }

    // guts
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

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token)
    }

    fn get_lexeme(&self) -> &'a str {
        self.source
            .get(self.start..self.current)
            .expect("self.start..self.current is not a valid slice of self.source")
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

    fn string(&mut self) -> Option<Token<'a>> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string");
            return Option::None;
        }

        self.advance();

        let string_value = self.get_lexeme();
        let string_literal = string_value.get(1..string_value.len() - 1).unwrap();
        Option::from(Token::new(
            TokenType::String,
            string_literal,
            Option::from(string_literal),
            Option::None,
            self.line,
        ))
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn number(&mut self) -> Option<Token<'a>> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let lexeme = self.get_lexeme();
        Option::from(Token::new(
            TokenType::Number,
            lexeme,
            Option::None,
            Option::from(lexeme.parse::<f64>().unwrap()),
            self.line,
        ))
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 > self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(self.current + 1)
                .expect("self.current + 1 is greater than the number of chars in self.source")
        }
    }
}
