mod errors;
mod lexer;
mod parser;
use parser::Parser;
fn main() {
    let mut parser = Parser::new();
    let program: &str = r#"let mdrrr = 55;"#;
    let ast = parser.init(program.to_owned());
    println!("{:?}", ast)
}
