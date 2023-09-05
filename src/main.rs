mod errors;
mod lexer;
mod parser;
mod typechecker;

use clap::{Parser, Subcommand};
use parser::Parser as KarmParser;
use std::{fs, process::exit};
use typechecker::TypeChecker;

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
        Some(Commands::Shell {}) => {}
        None => {}
    }
}

fn build(path: &String, cli: &Cli) {
    if path.ends_with(".kr") {
        let program = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(e) => panic!("{}", e),
        };
        let ast = KarmParser::new(program).program();
        if cli.ast == true {
            println!("{:#?}", ast);
        }

        // TODO: As we are here passing `ast` so its value is moved and not borrowed (but we don't want that...)
        println!("{:?}", TypeChecker::new(ast).init());
    } else {
        println!("This is not a valid Karm file! (.kr)");
        exit(1);
    }
}

fn _shell() {}
