#![allow(dead_code)]
mod ast;
mod codegen_c;
mod codegen_dot;
mod graph;
mod parser;
mod token;

use crate::graph::*;
use crate::parser::*;
use crate::token::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    // 1. Get Command Line Arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename.fsm>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);

    // 2. Read the file from disk
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    println!("Compiling {}...", filename);

    // 3. Lexing
    let mut lexer = Lexer::new(&code);
    let tokens: Vec<Token> = std::iter::from_fn(|| {
        let t = lexer.next_token();
        if t == Token::EOF { None } else { Some(t) }
    })
    .collect();

    // 4. Parsing
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_machine() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse Error: {}", e);
            process::exit(1);
        }
    };

    // 5. Semantic Analysis
    let graph = match FsmGraph::compile(ast) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Compilation Error: {}", e);
            process::exit(1);
        }
    };

    // 6. Generate Output Filenames
    let stem = path
        .file_stem()
        .unwrap_or_else(|| path.as_os_str())
        .to_str()
        .unwrap();
    let dot_file = format!("{}.dot", stem);
    let c_file = format!("{}.c", stem);

    // 7. Backend: Generate DOT File
    let dot_code = codegen_dot::generate_dot(&graph);
    fs::write(&dot_file, dot_code).expect("Unable to write DOT file!");
    println!("✓ Generated {}", dot_file);

    // 8. Backend: Generate C File
    let c_code = codegen_c::generate_c(&graph);
    fs::write(&c_file, c_code).expect("Unable to write C file!");
    println!("✓ Generated {}", c_file);
}
