pub mod expr;
mod parser;
mod scanner;
pub mod token;
pub mod token_type;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use self::{parser::Parser, scanner::Scanner};

pub struct Lox {}

impl Lox {
    pub fn run_file(&self, file_name: &str) {
        // lifetime of source is this block
        let source: String =
            fs::read_to_string(file_name).expect("Something went wrong reading the file");
        self.run(source.as_str(), false);
    }

    pub fn run_prompt(&self) {
        loop {
            // lifetime of line is this loop
            let mut line = String::new();
            print!("> ");
            io::stdout().flush().expect("something went wrong");
            io::stdin()
                .read_line(&mut line)
                .expect("something went wrong");
            // run borrows line
            self.run(line.as_str(), true);
        }
    }

    fn run(&self, source: &str, reset_errors: bool) {
        // lifetime of source depends on caller
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let parser = Parser::new(tokens);
        let expr = parser.expression();
        match expr {
            Ok(expr) => print!("{}", expr.print()),
            Err(err) => print!("{}", err),
        }
        if reset_errors {
            scanner.had_error = false;
            return;
        }

        if scanner.had_error {
            exit(65);
        }
    }
}
