#[derive(Debug, PartialEq, Clone)]
pub enum LexItem {
    //One character tokens
    LeftParen(LexItemInfo),
    RightParen(LexItemInfo),
    Plus(LexItemInfo),
    Minus(LexItemInfo),
    Slash(LexItemInfo),
    Star(LexItemInfo),
    LessThan(LexItemInfo),
    And(LexItemInfo),
    Not(LexItemInfo),
    StatementEnd(LexItemInfo),
    Equal(LexItemInfo),
    // One or Two character tokens
    Separator(LexItemInfo),

    //Two character tplem
    Range(LexItemInfo),
    Assign(LexItemInfo),

    //Literals
    StringLiteral(LexItemInfo),
    IntegerLiteral(LexItemInfo),

    //Keywords
    Var(LexItemInfo),
    For(LexItemInfo),
    End(LexItemInfo),
    In(LexItemInfo),
    Do(LexItemInfo),
    Read(LexItemInfo),
    Print(LexItemInfo),
    Int(LexItemInfo),
    String(LexItemInfo),
    Bool(LexItemInfo),
    Assert(LexItemInfo),

    Identifier(LexItemInfo),

    Default(LexItemInfo)
}
#[derive(Debug, PartialEq, Clone)]
pub struct LexItemInfo {
    pub text: String,
    pub line_number: i32,
    pub column_number: i32,
}

impl Default for LexItem {
    fn default() -> Self {LexItem::Default(LexItemInfo {text: "default".into(), line_number: -1, column_number: -1})}
}