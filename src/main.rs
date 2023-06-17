mod errors;
mod lexer;
mod parser;
mod typechecker;

use parser::Parser;
fn main() {
    let program: &str = r#"fn main :: n -> if n + 3 <= 1 + 4 ? n : 0;"#;
    let ast = Parser::new(program.to_owned()).program();
    println!("{:?}", ast)
}
