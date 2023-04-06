mod errors;
mod lexer;
mod parser;
use parser::Parser;
fn main() {
    let mut parser = Parser::new();
    let program: &str = r#"fn main :: n, x -> 55 + 2;"#;
    let ast = parser.init(program.to_owned());
    println!("{:?}", ast)
}
