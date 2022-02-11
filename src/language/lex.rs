#[derive(Debug)]
pub enum LexItem {
    //One character tokens
    LeftParen(LexItemInfo), RightParen(LexItemInfo),
    Plus(LexItemInfo), Minus(LexItemInfo), Slash(LexItemInfo),
    Star(LexItemInfo), LessThan(LexItemInfo), And(LexItemInfo),
    Not(LexItemInfo), StatementEnd(LexItemInfo),
    // One or Two character tokens
    Separator(LexItemInfo),

    //Two character tplem
    Range(LexItemInfo), Assign(LexItemInfo),

    //Literals    
    StringLiteral(LexItemInfo), IntegerLiteral(LexItemInfo),

    //Keywords
    Var(LexItemInfo), For(LexItemInfo), End(LexItemInfo), In(LexItemInfo), Do(LexItemInfo),
    Read(LexItemInfo), Print(LexItemInfo), Int(LexItemInfo), String(LexItemInfo), Bool(LexItemInfo),
    Assert(LexItemInfo),

    Identifier(LexItemInfo)
}
#[derive(Debug)]
pub struct LexItemInfo {
    pub text: String,
    pub line_number: i32,
    pub column_number: i32
}
