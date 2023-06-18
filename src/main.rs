mod errors;
mod lexer;
mod parser;
mod typechecker;

use clap::{Parser, Subcommand};
use parser::Parser as KarmParser;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    ast: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the specified file (see karm build --help)
    Build {
        file: String,
    },

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
        let program: &str = r#"use "std.kr";
    fn fib :: n -> if n <= 1 ? n : fib(n - 1) + fib(n - 2);"#;
        let ast = KarmParser::new(program.to_owned()).program();
        if cli.ast == true {
            println!("{:?}", ast);
        }
    } else {
        println!("This is not a valid Karm file! (.kr)");
        exit(1);
    }
}
