use clap::{Parser, Subcommand};
use crate::expr::AstPrinter;
use crate::lox_interpreter::Interpreter;
use crate::lox_tokenizer::LoxTokenizer;
use std::fs;
use std::io::{self, Write};
use std::process;

mod environment;
mod expr;
mod lox_interpreter;
mod lox_parser;
mod lox_tokenizer;
mod stmt;
mod token;
mod token_types;

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

fn main() {
    let cli = Cli::parse();

    let filename = match &cli.command {
        Commands::Tokenize { filename } => filename,
        Commands::Parse { filename } => filename,
        Commands::Evaluate { filename } => filename,
        Commands::Run { filename } => filename,
    };

    let file_contents = read_file(filename);
    if file_contents.is_empty() {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        process::exit(74);
    }

    writeln!(io::stderr(), "Read file with content: {} \n\n", file_contents).unwrap();

    match cli.command {
        Commands::Tokenize { .. } => {
            let mut tokenizer = LoxTokenizer::default();
            let result = tokenizer.tokenize(&file_contents);
            for token in result {
                writeln!(io::stdout(), "{}", token).unwrap();
            }
            if tokenizer.had_error {
                process::exit(65)
            };
        }
        Commands::Parse { .. } => {
            let mut lox_tokenizer = LoxTokenizer::default();
            let tokens = lox_tokenizer.tokenize(&file_contents);
            if lox_tokenizer.had_error {
                process::exit(65)
            }
            for token in tokens.clone() {
                writeln!(io::stderr(), "{}", token).unwrap();
            }
            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse_expression(); // Still use expression parsing for now
            if parser.has_error {
                process::exit(65);
            }
            println!("{}", expr.accept(&AstPrinter {}));
        }
        Commands::Evaluate { .. } => {
            let mut lox_tokenizer = LoxTokenizer::default();
            let tokens = lox_tokenizer.tokenize(&file_contents);
            if lox_tokenizer.had_error {
                process::exit(65)
            }
            for token in tokens.clone() {
                writeln!(io::stderr(), "{}", token).unwrap();
            }
            let mut parser = lox_parser::LoxParser::new(tokens);
            let expr = parser.parse_expression(); // Use expression parsing
            if parser.has_error {
                process::exit(65);
            }
            
            // Create interpreter and evaluate expression
            let interpreter = Interpreter::new();
            match interpreter.evaluate(&expr) {
                Ok(value) => {
                    println!("{}", value);
                }
                Err(error) => {
                    writeln!(io::stderr(), "{}", error).unwrap();
                    process::exit(70);
                }
            }
        }
        Commands::Run { .. } => {
            let mut lox_tokenizer = LoxTokenizer::default();
            let tokens = lox_tokenizer.tokenize(&file_contents);
            if lox_tokenizer.had_error {
                process::exit(65)
            }
            let mut parser = lox_parser::LoxParser::new(tokens);
            let statements = parser.parse(); // Use statement-based parsing
            if parser.has_error {
                process::exit(65);
            }
            
            // Create interpreter and execute statements
            let interpreter = Interpreter::new();
            match interpreter.interpret(&statements) {
                Ok(()) => {
                    // Successful execution - output was produced via print statements
                }
                Err(error) => {
                    writeln!(io::stderr(), "{}", error).unwrap();
                    process::exit(70);
                }
            }
        }
    }
}

fn read_file(filename: &String) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    })
}
