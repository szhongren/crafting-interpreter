mod environment;
pub mod expr;
mod interpreter;
mod parser;
mod scanner;
pub mod stmt;
pub mod token;
pub mod token_type;
pub mod value;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use crate::lox::interpreter::Interpreter;

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
            print!("\n");
        }
    }

    fn run(&self, source: &str, reset_errors: bool) {
        // lifetime of source depends on caller
        let mut scanner = Scanner::new(source);
        let result_tokens = scanner.scan_tokens();

        match result_tokens {
            Ok(ref tokens) => {
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(ref err) => {
                println!("{}", err);
                if !reset_errors {
                    exit(65);
                }
                return;
            }
        }

        let parser = Parser::new(result_tokens.expect("something went very wrong"));
        let stmts = parser.parse();
        match stmts {
            Ok(ref stmts) => {
                for stmt in stmts {
                    println!("{}", stmt);
                }
            }
            Err(err) => {
                println!("{}", err);
                if !reset_errors {
                    exit(65);
                }
                return;
            }
        }

        let mut interpreter = Interpreter::new();
        let value = interpreter.interpret(stmts.expect("failed at interpreting"));

        if let Err(err) = value {
            println!("{}", err);
            if !reset_errors {
                exit(65);
            }
        }
    }
}
