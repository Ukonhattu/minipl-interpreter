#[derive(Debug)]
pub enum LexItem {
    Parenthesis(char),
    Operand(char),
    Integer(String),
    Keyword(String),
    Identifier(String)
}


pub fn is_one_char_delimiter(ch: &char) -> bool {
    matches!(ch, ' ' | '+' | '-' | '*' | '/' | '<' | '&' | '!' | ';')
}

pub fn can_be_one_or_two_char_delimiter(ch: &char) -> bool {
    matches!(ch, ':')
}

pub fn is_two_char_delimiter(st: &str) -> bool {
    matches!(st, ":=")
}

pub fn is_start_of_delimeter(ch: &char) -> bool {
    is_one_char_delimiter(ch) || can_be_one_or_two_char_delimiter(ch)
}

pub fn is_operator(ch: &char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/' | '<' | '=' | '&' | '!')
}

pub fn is_valid_identifier(st: &str) -> bool {
    match st.chars().next() {
        Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' |'8' | '9')  => false,
        Some(ch) => !is_start_of_delimeter(&ch),
        _ => false
    }
}

pub fn is_keyword(st: &str) -> bool {
    matches!(st, "var" | "for" | "end" | "in" | "do" | "read" | "print" | "int" | "string" | "bool" | "assert")
}

pub fn is_integer(st: &str) -> bool {
    let chars: Vec<char> = st.chars().collect();
    if chars.len() == 0 { 
        return false;
    }
    for i in 0..chars.len() {
       if  !matches!(&chars[i], '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' |'8' | '9') || (chars[i] == '-' && i > 0) {
           return false;
       }
    }
    return true;

}

pub fn str_type(st: &String) -> LexItem {
    match st {
        _ => LexItem::Identifier("kekw".to_string())
    }
}
