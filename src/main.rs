#[macro_use]
extern crate lazy_static;

mod interpreter;

use std::env;
use std::process::exit;

use interpreter::Lox;

fn main() {
    let lox = Lox {};
    let args: Vec<String> = env::args().collect();
    // different from go, first arg is always binary in rust
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64)
    } else if args.len() == 2 {
        // lend args[1] to run_file
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
