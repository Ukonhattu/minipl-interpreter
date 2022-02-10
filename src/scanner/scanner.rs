use super::token_checker::{self, can_be_one_or_two_char_delimiter, is_one_char_delimiter, is_two_char_delimiter};
use super::token_checker::LexItem;
pub struct Scanner {
    program: String,
}

impl Scanner {
    
    pub fn new(program: String) -> Self {
        Self {
            program: program,
        }
    }
    pub fn scan(&self) -> Result<Vec<LexItem>, String> {
        let mut it = self.program.chars().peekable();
        let mut result: Vec<LexItem> = Vec::new();
        while let Some(c) = it.next() {
            match c {
                ' ' | '+' | '-' | '*' | '/' | '<' | '&' | '!' | ';' | '(' | ')' => { // TODO remove whitespaces from operands
                    result.push(LexItem::Operand(c))
                }
                ':' => {
                    match it.peek() {
                        Some('=') => {
                            result.push(LexItem::Keyword(":=".to_string()));// TODO Keyword as a placeholder!!!
                            it.next();
                        }
                        _ => {
                            result.push(LexItem::Keyword(":".to_string())) // TODO Keyword as a placeholder!!!
                        }
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut number = c.to_string();
                    loop {
                        
                        match it.peek() {
                            Some(n) if matches!(n, '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => { // 0 at a start? -- No decimals in minipl!

                            }
                            _ => {
                                result.push(LexItem::Integer(number.to_string()));
                                break;
                            }
                        }
                        number += &it.next().unwrap().to_string(); // TODO this is bad practice but I know that it is valid, perhaps fix
                    }
                }
                _ => {
                    let mut st = c.to_string();
                    loop {
                        
                        match it.peek() {
                            Some(n) if matches!(n, ' ' | '+' | '-' | '*' | '/' | '<' | '&' | '!' | ';' | ':') => {
                                result.push(LexItem::Identifier(st.to_string()));
                                break;
                            }
                            Some(_) => {

                            }
                            None => {
                                result.push(LexItem::Identifier(st.to_string()));
                                break;
                            }
                        }
                        st += &it.next().unwrap().to_string(); // TODO this is bad practice but I know that it is valid, perhaps fix
                    }
                }
            }
        }
        Ok(result)
    }




}

