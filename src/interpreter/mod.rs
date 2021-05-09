mod scanner;
mod token;
mod token_type;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use self::scanner::Scanner;

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
        for token in scanner.scan_tokens() {
            println!("{:?}", token);
        }

        if reset_errors {
            scanner.had_error = false;
            return;
        }

        if (scanner.had_error) {
            exit(65);
        }
    }
}
