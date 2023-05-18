mod errors;
mod lexer;
mod parser;
use parser::Parser;
fn main() {
    let program: &str = r#"fn main -> main(25 - 33) + 2;"#;
    let ast = Parser::new(program.to_owned()).program();
    println!("{:?}", ast)
}
