use scanner::scanner::Scanner;
use std::env;
use std::fs;

mod scanner;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let _scanner = Scanner::new(contents);
    
}
