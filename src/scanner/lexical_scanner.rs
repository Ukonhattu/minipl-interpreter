use crate::language::lex::{LexItem, LexItemInfo};
pub struct Scanner {
    program: String,
}

impl Scanner {
    pub fn new(program: String) -> Self {
        Self { program }
    }
    pub fn scan(&self) -> Result<Vec<LexItem>, String> {
        let mut it = self.program.chars().peekable();
        let mut result: Vec<LexItem> = Vec::new();
        let mut line_number = 1;
        let mut column_number = 0;
        while let Some(c) = Scanner::advance(&mut it, &mut column_number) {
            match c {
                ' ' | '+' | '-' | '*' | '<' | '&' | '!' | ';' | '(' | ')' | '\n' | '\r' => {
                    //Detect one character delimeters
                    match c {
                        '(' | ')' => result.push(LexItem::Parenthesis(LexItemInfo{text: c.to_string(), line_number, column_number})),
                        ';' => result.push(LexItem::StatementEnd(LexItemInfo{text: c.to_string(), line_number, column_number})),
                        '\n' => {
                            line_number += 1;
                            column_number = 0;
                        }
                        ' ' | '\r' => (),
                        _ => result.push(LexItem::Operator(LexItemInfo{text: c.to_string(), line_number, column_number})),
                    }
                }
                ':' => {
                    // is it : or :=
                    match it.peek() {
                        Some('=') => {
                            result.push(LexItem::Assign(LexItemInfo{text: ":=".to_string(), line_number, column_number}));
                            Scanner::advance(&mut it, &mut column_number);
                        }
                        _ => result.push(LexItem::Separator(LexItemInfo{text: ":".to_string(), line_number, column_number})),
                    }
                }
                '.' => {
                    if let Some(n) = it.peek() {
                        match n {
                            '.' => {
                                result.push(LexItem::Range(LexItemInfo{text: ":".to_string(), line_number, column_number}));
                                Scanner::advance(&mut it, &mut column_number);
                            }
                            _ => {
                                return Err(format!("Unexpected character {}, line {}, column {}", c, line_number, column_number));
                                // Everything else we can take but one damn comma is a no! (Add decimal later)
                            }
                        }
                    }
                }
                '/' => {
                    // Detect comment blocks (Skip the rest of the line if "//")
                    if let Some(n) = it.peek() {
                        match n {
                            '/' => {
                                for m in it.by_ref() {
                                    if m == '\n' {
                                        line_number += 1;
                                        break;
                                    }
                                }
                            }
                            _ => result.push(LexItem::Operator(LexItemInfo{text: c.to_string(), line_number, column_number}))
                        }
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    //is it number (ints only so far)
                    let mut number = c.to_string();
                    while let Some(n) = it.peek() {
                        match n {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
                            _ => {
                                result.push(LexItem::Integer(LexItemInfo{text: c.to_string(), line_number, column_number}));
                                break;
                            }
                        }
                        number += &Scanner::advance(&mut it, &mut column_number).unwrap_or_default().to_string();
                    }
                }
                '"' => {
                    //Strings as one token
                    let mut st = String::new();
                    while let Some(n) = Scanner::advance(&mut it, &mut column_number) {
                        match n {
                            '"' => break,
                            '\\' => {
                                if let Some(m) = it.peek() {
                                    match m {
                                        '\\' => {
                                            st += &m.to_string();
                                            Scanner::advance(&mut it, &mut column_number);
                                        }
                                        _ => {
                                            st += &n.to_string();
                                            st += &m.to_string();
                                            Scanner::advance(&mut it, &mut column_number);
                                        }
                                    }
                                }
                            }
                            _ => st += &n.to_string(),
                        }
                    }
                    result.push(LexItem::String(LexItemInfo{text: st.to_string(), line_number, column_number}))
                }
                _ => {
                    // is it keyword? if not, then it is an identifier
                    let mut st = c.to_string();
                    loop {
                        match it.peek() {
                            Some(n)
                                if matches!(
                                    n,
                                    ' ' | '+'
                                        | '-'
                                        | '*'
                                        | '/'
                                        | '<'
                                        | '&'
                                        | '!'
                                        | ';'
                                        | ':'
                                        | '.'
                                        | '('
                                        | ')'
                                        | '\n'
                                        | '\r'
                                ) =>
                            {
                                match st.as_str() {
                                    "var" | "for" | "end" | "in" | "do" | "read" | "print"
                                    | "int" | "string" | "bool" | "assert" => {
                                        result.push(LexItem::Keyword(LexItemInfo{text: st.to_string(), line_number, column_number}))
                                    }
                                    _ => result.push(LexItem::Identifier(LexItemInfo{text: st.to_string(), line_number, column_number})),
                                }
                                break;
                            }
                            Some(_) => (),
                            None => {
                                result.push(LexItem::Identifier(LexItemInfo{text: st.to_string(), line_number, column_number}));
                                break;
                            }
                        }
                        st += &Scanner::advance(&mut it, &mut column_number).unwrap_or_default().to_string();
                    }
                }
            }
        }
        Ok(result)
    }

    fn advance(it: &mut impl Iterator<Item = char>, column_number: &mut i32) -> Option<char> {
        *column_number = *column_number + 1;
        it.next()
    }
}
