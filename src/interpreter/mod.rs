mod runner;
use std::{
    fs,
    io::{self, Write},
};

use self::runner::run;

pub fn run_file(file_name: &str) {
    // lifetime of source is this block
    let source: String =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    run(source.as_str());
}

pub fn run_prompt() {
    loop {
        // lifetime of line is this loop
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().expect("something went wrong");
        io::stdin()
            .read_line(&mut line)
            .expect("something went wrong");
        // run borrows line
        run(line.as_str())
    }
}
