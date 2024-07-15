mod errors;
mod lexer;
mod parser;
// mod repl;
// mod typechecker;

use clap::{Parser, Subcommand};
use parser::Parser as KarmParser;
use core::panic;
use std::{fs, process::exit};
// use typechecker::TypeChecker;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print the result AST
    #[arg(short, long)]
    ast: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the specified file (see karm build --help)
    Build { file: String },

    /// Interpret the input using a shell
    Shell {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build { file }) => build(file, &cli),
        Some(Commands::Shell {}) => /* _shell() */ panic!("Shell not implemented!"),
        None => {}
    }
}

fn build(path: &String, cli: &Cli) {

    if !path.ends_with(".kr") {
        println!("This is not a valid Karm file! (.kr)");
        exit(1);
    }  

    let program = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(e) => panic!("{e}"),
    };

    let ast = match KarmParser::new(&program).program() {
        Ok(ast) => ast,
        Err(err) => {
            println!("{err}");
            exit(1)
        }
    };

    if cli.ast {
        println!("{:#?}", ast);
    }

    // TODO: As we are here passing `ast` so its value is moved and not borrowed (but we don't want that...)
    // println!("{:?}", TypeChecker::new(ast).init());
}

/* 
fn _shell() {
    let session = repl::Repl::new(">>> ".to_string(), "... ".to_string(), Vec::new());
    session.run();
}
*/