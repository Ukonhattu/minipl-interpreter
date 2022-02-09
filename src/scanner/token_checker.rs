

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
