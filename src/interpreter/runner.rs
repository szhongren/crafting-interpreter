use self::scanner::Scanner;

mod scanner;

pub fn run(source: &str) {
    // lifetime of source depends on caller
    let scanner = Scanner { source: source };
    for token in scanner.scan_tokens() {
        println!("{}", token);
    }
}
