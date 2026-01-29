use crate::expr::AstPrinter;
use crate::lox_interpreter::Interpreter;
use crate::lox_tokenizer::LoxTokenizer;
use crate::token::Token;
use clap::{Parser, Subcommand};
use std::fs;
use std::process;

mod environment;
mod error;
mod expr;
mod lox_interpreter;
mod lox_parser;
mod lox_tokenizer;
mod stmt;
mod token;
mod token_types;
mod value;

#[derive(Parser)]
#[command(name = "lox")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tokenize { filename: String },
    Parse { filename: String },
    Evaluate { filename: String },
    Run { filename: String },
}

fn tokenize_or_exit(contents: &str) -> Vec<Token> {
    let mut tokenizer = LoxTokenizer::default();
    let tokens = tokenizer.tokenize(contents);
    if tokenizer.had_error {
        process::exit(65);
    }
    tokens
}

fn log_tokens_to_stderr(tokens: &[Token]) {
    for token in tokens {
        eprintln!("{}", token);
    }
}

fn main() {
    let cli = Cli::parse();

    let filename = match &cli.command {
        Commands::Tokenize { filename }
        | Commands::Parse { filename }
        | Commands::Evaluate { filename }
        | Commands::Run { filename } => filename,
    };

    let file_contents = read_file(filename);
    if file_contents.is_empty() {
        eprintln!("Failed to read file {}", filename);
        process::exit(74);
    }

    eprintln!("Read file with content: {} \n\n", file_contents);

    match cli.command {
        Commands::Tokenize { .. } => {
            let mut tokenizer = LoxTokenizer::default();
            let tokens = tokenizer.tokenize(&file_contents);
            for token in &tokens {
                println!("{}", token);
            }
            if tokenizer.had_error {
                process::exit(65);
            }
        }
        Commands::Parse { .. } => {
            let tokens = tokenize_or_exit(&file_contents);
            log_tokens_to_stderr(&tokens);

            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse_expression();
            if parser.has_error {
                process::exit(65);
            }
            println!("{}", expr.accept(&AstPrinter {}));
        }
        Commands::Evaluate { .. } => {
            let tokens = tokenize_or_exit(&file_contents);
            log_tokens_to_stderr(&tokens);

            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse_expression();
            if parser.has_error {
                process::exit(65);
            }

            let interpreter = Interpreter::new();
            match interpreter.evaluate(&expr) {
                Ok(value) => println!("{}", value),
                Err(error) => {
                    eprintln!("{}", error);
                    process::exit(70);
                }
            }
        }
        Commands::Run { .. } => {
            let tokens = tokenize_or_exit(&file_contents);

            let mut parser = lox_parser::LoxParser::new(tokens);
            let statements = parser.parse();
            if parser.has_error {
                process::exit(65);
            }

            let interpreter = Interpreter::new();
            if let Err(error) = interpreter.interpret(&statements) {
                eprintln!("{}", error);
                process::exit(70);
            }
        }
    }
}

fn read_file(filename: &String) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    })
}
