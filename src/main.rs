mod ast;
mod error;
mod optimizations;
mod parser;
mod runtime;
mod tokenizer;

use ast::Ir;
use error::BfcError;
use optimizations::Optimizations;
use parser::Parser;
use runtime::Runtime;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("Please specify a file");

    let start = Instant::now();
    let ir = parse(file_path, args.get(2).is_some()).unwrap();
    let parsing_time = start.elapsed();

    Runtime::new(std::io::stdout().lock()).execute(&ir);
    let running_time = start.elapsed();

    println!("\n\nParsing time: {}us", parsing_time.as_micros());
    println!(
        "Running time: {}us",
        running_time.as_micros() - parsing_time.as_micros()
    );
}

fn parse(file_path: &str, optimize: bool) -> Result<Vec<Ir>, BfcError> {
    let mut open_options = OpenOptions::new();
    let handle = open_options.read(true).open(file_path).unwrap();

    let buf_reader = BufReader::new(handle);
    let parser = if optimize {
        Parser::new_with(Optimizations::new_enable_all())
    } else {
        Parser::new()
    };

    parser.parse(buf_reader)
}
