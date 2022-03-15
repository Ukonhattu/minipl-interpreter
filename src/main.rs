use interpreter::interpreter::Interpreter;
use parser::syntax_parser::SyntaxParser;
use scanner::lexical_scanner::Scanner;
use std::env;
use std::fs;
mod language;
mod parser;
mod interpreter;
mod data_structures;

mod scanner;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let scanner = Scanner::new(contents);
    

    let scan_result = scanner.scan();
    //println!("{:#?}", scan_result);
    let mut parser = SyntaxParser::new(scan_result.unwrap());
    let parse_result = parser.parse();
    //println!("{:#?}", parser.parse());
    let mut interpreter = Interpreter::new(parse_result);
    interpreter.interpret();

}

