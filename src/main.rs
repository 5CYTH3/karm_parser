mod errors;
mod lexer;
mod parser;
mod typechecker;

use parser::Parser;
fn main() {
    let program: &str = r#"use "std.kr"
fn fib :: n -> if n <= 1 ? n : fib(n - 1) + fib(n - 2);"#;
    let ast = Parser::new(program.to_owned()).program();
    println!("{:?}", ast)
}
