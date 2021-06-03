#[macro_use]
extern crate lazy_static;

mod java_class_generator;
mod lox;

use std::process::exit;
use std::{env, io::Result};

use java_class_generator::define_ast;
use lox::Lox;

use lox::expr::Expr;
use lox::token::Token;

use lox::token_type::TokenType;

fn main() -> Result<()> {
    let lox = Lox {};
    let args: Vec<String> = env::args().collect();
    // different from go, first arg is always binary in rust
    if args.len() > 2 {
        match args[1].as_str() {
            "gen" => {
                define_ast(
                    &args[2],
                    "Expr",
                    vec![
                        "Assign: Token name, Expr value",
                        "Binary: Expr left, Token operator, Expr right",
                        "Call: Expr callee, Token paren, List<Expr> arguments",
                        "Grouping: Expr expression",
                        "Literal: Object value",
                        "Logical: Expr left, Token operator, Expr right",
                        "Urnary: Token operator, Expr right",
                        "Variable: Token name",
                    ],
                )?;
                define_ast(
                    &args[2],
                    "Stmt",
                    vec![
                        "Block: List<Stmt> statements",
                        "Expression: Expr expression",
                        "Function: Token name, List<Token> params, List<Stmt> body",
                        "If: Expr condition, Stmt thenBranch, Stmt elseBranch",
                        "Print: Expr expression",
                        "Return: token keyword, Expr value",
                        "Var: Token name, Expr initializer",
                        "While: Expr condition, Stmt body",
                    ],
                )?
            }
            _ => {
                println!("Usage: jlox [script]");
                exit(64)
            }
        }
    } else if args.len() == 2 {
        // lend args[1] to run_file
        match args[1].as_str() {
            "ast" => {
                let expression = Expr::Binary(
                    Box::from(Expr::Urnary(
                        Token::new(TokenType::Minus, "-".to_string(), None, 1),
                        Box::from(Expr::NumberLiteral(123 as f64)),
                    )),
                    Token::new(TokenType::Star, "*".to_string(), None, 1),
                    Box::from(Expr::Grouping(Box::from(Expr::NumberLiteral(45.67 as f64)))),
                );
                print!("{}", expression);
            }
            _ => lox.run_file(&args[1]),
        }
    } else {
        lox.run_prompt();
    }
    Ok(())
}
