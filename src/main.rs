mod errors;
mod lexer;
mod parser;
use parser::Parser;
fn main() {
    let program: &str = r#"fn fib :: n -> if n <= 2 ? n : fib;"#;
    let ast = Parser::new(program.to_owned()).program();
    println!("{:?}", ast)
}
