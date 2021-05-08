mod interpreter;

use std::env;
use std::process::exit;

use interpreter::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();
    // different from go, first arg is always binary in rust
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64)
    } else if args.len() == 2 {
        // lend args[1] to run_file
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
