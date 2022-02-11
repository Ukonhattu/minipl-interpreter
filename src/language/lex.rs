#[derive(Debug)]
pub enum LexItem {
    Parenthesis(LexItemInfo),
    Operator(LexItemInfo),
    Integer(LexItemInfo),
    String(LexItemInfo),
    Keyword(LexItemInfo),
    Identifier(LexItemInfo),
    Assign(LexItemInfo),
    StatementEnd(LexItemInfo),
    Separator(LexItemInfo),
    Range(LexItemInfo),
}
#[derive(Debug)]
pub struct LexItemInfo {
    pub text: String,
    pub line_number: i32,
    pub column_number: i32
}
