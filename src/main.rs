mod errors;
mod lexer;
mod parser;
use parser::Parser;
fn main() {
    let program: &str = r#"fn main :: n -> if n <= 1 ? n : 0;"#;
    let ast = Parser::new(program.to_owned()).program();
    println!("{:?}", ast)
}
