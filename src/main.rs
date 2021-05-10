#[macro_use]
extern crate lazy_static;

mod interpreter;
mod java_class_generator;

use std::process::exit;
use std::{env, io::Result};

use interpreter::Lox;
use java_class_generator::define_ast;

fn main() -> Result<()> {
    let lox = Lox {};
    let args: Vec<String> = env::args().collect();
    // different from go, first arg is always binary in rust
    if args.len() > 2 {
        if &args[1] == "gen" {
            define_ast(
                &args[2],
                "Expr",
                vec![
                    "Binary: Expr left, Token operator, Expr right",
                    "Grouping: Expr expression",
                    "Literal: Object value",
                    "Urnary: Token operator, Expr right",
                ],
            )?
        } else {
            println!("Usage: jlox [script]");
            exit(64)
        }
    } else if args.len() == 2 {
        // lend args[1] to run_file
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
    Ok(())
}
