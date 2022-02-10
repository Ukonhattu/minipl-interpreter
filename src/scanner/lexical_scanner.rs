use crate::language::{lex::LexItem};
pub struct Scanner {
    program: String,
}

impl Scanner {
    
    pub fn new(program: String) -> Self {
        Self {
            program,
        }
    }
    pub fn scan(&self) -> Result<Vec<LexItem>, String> {
        let mut it = self.program.chars().peekable();
        let mut result: Vec<LexItem> = Vec::new();
        while let Some(c) = it.next() {
            match c {
                ' ' | '+' | '-' | '*' | '<' | '&' | '!' | ';' | '(' | ')' | '\n' | '\r' => { //Detect one character delimeters
                    match c {
                        '(' | ')' =>  result.push(LexItem::Parenthesis(c)),                   
                        ';' => result.push(LexItem::StatementEnd(c)),
                        ' ' | '\n' | '\r' => (),
                        _ => result.push(LexItem::Operator(c))
                    }                    
                }
                ':' => { // is it : or :=
                    match it.peek() {
                        Some('=') => {
                            result.push(LexItem::Assign(":=".to_string()));
                            it.next();
                        }
                        _ => {
                            result.push(LexItem::Separator(':'))
                        }
                    }
                }
                '.' => {
                    if let Some(n) = it.peek() {
                        match n {
                            '.' => {
                                result.push(LexItem::Range("..".to_string()));
                                it.next();
                            }
                            _ => {
                                return Err(format!("Unexpected character {}", c)) // Everything else we can take but one damn comma is a no! (Add decimal later)
                            }
                        }
                    }
                }
                '/' => { // Detect comment blocks (Skip the rest of the line if "//")
                    match it.peek() {
                        Some('/') => {
                            for n in it.by_ref() {
                                if n == '\n' {
                                    break;
                                }
                            }
                        }
                        _ => {
                            result.push(LexItem::Operator(c))
                        }
                    }                   
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { //is it number (ints only so far)
                    let mut number = c.to_string();
                    while let Some(n) = it.peek() {
                        match n {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
                            _ => {
                                result.push(LexItem::Integer(number.to_string()));
                                break;
                            }
                        }
                        number += &it.next().unwrap_or_default().to_string();
                    }
                }
                '"' => { //Strings as one token
                    let mut st = String::new();
                    
                    while let Some(n) = it.next() {
                        match n {
                            '"' => break,
                            '\\' => {
                                if let Some(m) = it.peek() {
                                    match m {
                                        '\\' => {
                                            st += &m.to_string();
                                            it.next();
                                        }

                                        _ => {
                                            st += &n.to_string();
                                            st += &m.to_string();
                                            it.next();
                                        }
                                    }
                                }
                            }
                            _ => {
                                st += &n.to_string()
                            }
                        }
                    }
                    result.push(LexItem::String(st))                  
                }
                _ => { // is it keyword? if not, then it is an identifier
                    let mut st = c.to_string();
                    loop {                      
                        match it.peek() {
                            Some(n) if matches!(n, ' ' | '+' | '-' | '*' | '/' | '<' | '&' | '!' | ';' | ':' | '.' | '(' | ')' | '\n' | '\r') => {
                                match st.as_str() {
                                    "var" | "for" | "end" | "in" | "do" | "read" | "print" | "int" | "string" | "bool" | "assert" => result.push(LexItem::Keyword(st.to_string())),
                                    _ => result.push(LexItem::Identifier(st.to_string()))
                                } 
                                break;
                            }
                            Some(_) => (),
                            None => {
                                result.push(LexItem::Identifier(st.to_string()));
                                break;
                            }
                        }
                        st += &it.next().unwrap_or_default().to_string();
                    }
                }
            }
        }
        Ok(result)
    } 
}

