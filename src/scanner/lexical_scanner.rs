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
                ' ' | '+' | '-' | '*' | '<' | '&' | '!' | ';' | '(' | ')' | '\n' | '\r' => {
                    match c {
                        '(' | ')' =>  result.push(LexItem::Parenthesis(c)),                   
                        ';' => result.push(LexItem::StatementEnd(c)),
                        ' ' | '\n' | '\r' => (),
                        _ => result.push(LexItem::Operator(c))
                    }
                    
                }
                ':' => {
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
                '/' => {
                    match it.peek() {
                        Some('/') => {
                            while let Some(n) = it.next() {
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
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut number = c.to_string();
                    loop {  
                        match it.peek() {
                            Some(n) if matches!(n, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => (),// 0 at a start? -- No decimals in minipl!
                            _ => {
                                result.push(LexItem::Integer(number.to_string()));
                                break;
                            }
                        }
                        number += &it.next().unwrap_or_default().to_string();
                    }
                }
                '"' => {
                    let mut st = String::new();
                    
                    while let Some(n) = it.next() {
                        match n {
                            '"' => break,
                            _ => {
                                st += &n.to_string()
                            }
                        }
                    }
                    result.push(LexItem::String(st))
                    
                }
                _ => {
                    let mut st = c.to_string();
                    loop {
                        
                        match it.peek() {
                            Some(n) if matches!(n, ' ' | '+' | '-' | '*' | '/' | '<' | '&' | '!' | ';' | ':') => {
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

