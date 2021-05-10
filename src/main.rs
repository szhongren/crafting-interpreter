#[macro_use]
extern crate lazy_static;

mod interpreter;
mod java_class_generator;

use std::process::exit;
use std::{env, io::Result};

use interpreter::Lox;
use java_class_generator::define_ast;

use interpreter::expr::Expr;
use interpreter::token::Token;

use interpreter::token_type::TokenType;

fn main() -> Result<()> {
    let lox = Lox {};
    let args: Vec<String> = env::args().collect();
    // different from go, first arg is always binary in rust
    if args.len() > 2 {
        match args[1].as_str() {
            "gen" => define_ast(
                &args[2],
                "Expr",
                vec![
                    "Binary: Expr left, Token operator, Expr right",
                    "Grouping: Expr expression",
                    "Literal: Object value",
                    "Urnary: Token operator, Expr right",
                ],
            )?,
            _ => {
                println!("Usage: jlox [script]");
                exit(64)
            }
        }
    } else if args.len() == 2 {
        // lend args[1] to run_file
        match args[1].as_str() {
            "ast" => {
                let expression = Expr::Binary {
                    left: Box::from(Expr::Urnary {
                        operator: Token::new(TokenType::Minus, "-", Option::None, Option::None, 1),
                        right: Box::from(Expr::Literal {
                            literal_type: interpreter::expr::LiteralType::NumberLiteral,
                            string_literal: Option::None,
                            number_literal: Option::Some(123 as f64),
                        }),
                    }),
                    operator: Token::new(TokenType::Star, "*", Option::None, Option::None, 1),
                    right: Box::from(Expr::Grouping {
                        expression: Box::from(Expr::Literal {
                            literal_type: interpreter::expr::LiteralType::NumberLiteral,
                            string_literal: Option::None,
                            number_literal: Option::Some(45.67),
                        }),
                    }),
                };
                print!("{}", expression.print());
            }
            _ => lox.run_file(&args[1]),
        }
    } else {
        lox.run_prompt();
    }
    Ok(())
}
