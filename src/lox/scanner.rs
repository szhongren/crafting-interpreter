use super::{
    token::{Literal, Token},
    token_type::TokenType,
};

use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS_MAP: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

pub struct Scanner<'a> {
    pub source: &'a str,
    tokens: Vec<Token>,
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

    // entry point
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Option::None,
            self.line,
        ));
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // parser
    fn scan_token(&mut self) -> Result<(), String> {
        let ch = self.advance();
        let maybe_token = match ch {
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
            '"' => {
                let result_string = self.string()?;
                Option::from(result_string)
            }
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ch => {
                return Err(String::from("unrecognized character"));
            }
        };

        if maybe_token.is_some() {
            self.add_token(maybe_token.unwrap());
        };
        Ok(())
    }

    fn generate_token_option(&self, token_type: TokenType) -> Option<Token> {
        Option::from(self.generate_new_token(token_type))
    }

    fn generate_new_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.get_lexeme(), Option::None, self.line)
    }

    // guts
    fn advance(&mut self) -> char {
        let ch = self.get_current_char();
        self.current += 1;
        ch
    }

    fn get_current_char(&self) -> char {
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .expect("self.current is greater than the number of chars in self.source");
        ch
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn get_lexeme(&self) -> String {
        let range = self.start..self.current;
        self.source
            .get(range)
            .expect("self.start..self.current is not a valid slice of self.source")
            .to_string()
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_current_char()
        }
    }

    fn peek_next(&self) -> char {
        let next_index = self.current + 1;
        if next_index >= self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(next_index)
                .expect("self.current + 1 is greater than the number of chars in self.source")
        }
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_alpha(ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch == '_')
    }

    fn is_alphanumeric(ch: char) -> bool {
        Self::is_alpha(ch) || Self::is_digit(ch)
    }

    fn number(&mut self) -> Option<Token> {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // can't do is_numeric because we can only have 1 decimal point in a number
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        let number_literal = self.get_lexeme();
        Option::from(Token::new(
            TokenType::Number,
            number_literal.clone(),
            Option::from(Literal::Number(number_literal.parse::<f64>().unwrap())),
            self.line,
        ))
    }

    fn string(&mut self) -> Result<Token, String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        // swallow closing quotes
        self.advance();

        let string_value = self.get_lexeme();
        Ok(Token::new(
            TokenType::String,
            string_value.clone(),
            Option::from(Literal::String(string_value)),
            self.line,
        ))
    }

    fn identifier(&mut self) -> Option<Token> {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let lexeme = self.get_lexeme();
        let maybe_token_type = KEYWORDS_MAP.get(lexeme.as_str());

        self.generate_token_option(match maybe_token_type {
            Some(&token_type) => token_type,
            None => TokenType::Identifier,
        })
    }
}
