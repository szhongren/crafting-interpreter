mod scanner;
mod token;
mod token_type;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use self::scanner::Scanner;

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn run_file(&self, file_name: &str) {
        // lifetime of source is this block
        let source: String =
            fs::read_to_string(file_name).expect("Something went wrong reading the file");
        self.run(source.as_str());

        if self.had_error {
            exit(65)
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            // lifetime of line is this loop
            let mut line = String::new();
            print!("> ");
            io::stdout().flush().expect("something went wrong");
            io::stdin()
                .read_line(&mut line)
                .expect("something went wrong");
            // run borrows line
            self.run(line.as_str());
            self.had_error = false;
        }
    }

    fn run(&self, source: &str) {
        // lifetime of source depends on caller
        let mut scanner = Scanner::new(source);
        for token in scanner.scan_tokens() {
            println!("{:?}", token);
        }
    }

    pub fn _error(&mut self, line_number: i32, message: &str) {
        self._report(line_number, "", message);
    }

    fn _report(&mut self, line_number: i32, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line_number, location, message);
        self.had_error = true;
    }
}
