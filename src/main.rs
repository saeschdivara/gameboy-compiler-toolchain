mod lexer;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// lexer (tokens) > ast (expressions/statements) > parser

fn main() {

    let arguments: Vec<String> = env::args().collect();
    if arguments.len() == 1 {
        println!("Path is missing as argument");
        return;
    }

    let path  = &arguments[1];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let f: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let start = Instant::now();
    let tokens = lexer::lex_content(f);
    let duration = start.elapsed();
    println!("Lex content: {:?}", duration);
    for token in tokens {
        println!("{:?}", token)
    }
}
