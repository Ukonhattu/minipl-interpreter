#[derive(Debug)]
pub enum LexItem {
    Parenthesis(char),
    Operator(char),
    Integer(String),
    Keyword(String),
    Identifier(String),
    Assign(String),
    StatementEnd(char),
    Separator(char)
}