use super::{token::Token, token_type::TokenType};

use std::{cell::RefCell, collections::HashMap};

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
    tokens: RefCell<Vec<Token<'a>>>,
    start: RefCell<usize>,
    current: RefCell<usize>,
    line: RefCell<i32>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: RefCell::from(Vec::new()),
            start: RefCell::from(0),
            current: RefCell::from(0),
            line: RefCell::from(1),
        }
    }

    // entry point
    pub fn scan_tokens(&'a self) -> Result<Vec<Token<'a>>, String> {
        while !self.is_at_end() {
            self.start.replace(*self.current.borrow());
            self.scan_token()?;
        }
        self.tokens.replace_with(|tokens| {
            tokens.push(Token::new(
                TokenType::Eof,
                "",
                Option::None,
                Option::None,
                *self.line.borrow(),
            ));
            tokens.to_vec()
        });
        Ok((*self.tokens.take()).to_vec())
    }

    fn is_at_end(&self) -> bool {
        *self.current.borrow() >= self.source.len()
    }

    // parser
    fn scan_token(&'a self) -> Result<(), String> {
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
                self.line.replace_with(|&mut old_line| old_line + 1);
                Option::None
            }
            '"' => {
                let result_string = self.string()?;
                Option::from(result_string)
            }
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ch => {
                return Err("unrecognized character".to_string());
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
        Token::new(
            token_type,
            self.get_lexeme(),
            Option::None,
            Option::None,
            *self.line.borrow(),
        )
    }

    // guts
    fn advance(&self) -> char {
        let ch = self.get_current_char();
        self.current
            .replace_with(|&mut old_current| old_current + 1);
        ch
    }

    fn get_current_char(&self) -> char {
        let ch = self
            .source
            .chars()
            .nth(*self.current.borrow())
            .expect("self.current is greater than the number of chars in self.source");
        ch
    }

    fn add_token(&self, token: Token<'a>) {
        self.tokens.replace_with(|tokens| {
            tokens.push(token);
            tokens.to_vec()
        });
    }

    fn get_lexeme(&self) -> &str {
        let range = *self.start.borrow()..*self.current.borrow();
        self.source
            .get(range)
            .expect("self.start..self.current is not a valid slice of self.source")
    }

    fn match_char(&self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_current_char() != ch {
            return false;
        }

        self.current
            .replace_with(|&mut old_current| old_current + 1);
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_current_char()
        }
    }

    fn string(&self) -> Result<Token, String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line.replace_with(|&mut old_line| old_line + 1);
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        // swallow closing quotes
        self.advance();

        let string_value = self.get_lexeme();
        let string_literal = string_value.get(1..string_value.len() - 1).unwrap();
        Ok(Token::new(
            TokenType::String,
            string_literal,
            Option::from(string_literal),
            Option::None,
            *self.line.borrow(),
        ))
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

    fn number(&self) -> Option<Token> {
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
            number_literal,
            Option::None,
            Option::from(number_literal.parse::<f64>().unwrap()),
            *self.line.borrow(),
        ))
    }

    fn peek_next(&self) -> char {
        let next_index = *self.current.borrow() + 1;
        if next_index >= self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(next_index)
                .expect("self.current + 1 is greater than the number of chars in self.source")
        }
    }

    fn identifier(&self) -> Option<Token> {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let lexeme = self.get_lexeme();
        let maybe_token_type = KEYWORDS_MAP.get(lexeme);

        self.generate_token_option(match maybe_token_type {
            Some(&token_type) => token_type,
            None => TokenType::Identifier,
        })
    }
}
