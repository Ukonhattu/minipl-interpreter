
#[derive(Debug, PartialEq, Clone)]
pub enum AstItem {
    Assign,
    Variable(VariableInfo),
    Constant(ConstantInfo),
    Not,
    BinOp(BinOpType),
    Read,
    Print,
    Assert,
    Block,
    For,
    Range,

    Root,
    Default
}
impl Default for AstItem {
    fn default() -> Self {AstItem::Default}
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub var_type: VariableType,
    pub source_info: SourceInfo
}

#[derive(Debug, PartialEq, Clone)]
pub enum  BinOpType {
    Plus,
    Minus,
    Multiply,
    Divide,
    LessThan,
    Equal,
    And
}

#[derive(Debug, PartialEq, Clone)]
pub struct  ConstantInfo {
    pub value: String,
    pub const_type: VariableType,
    pub source_info: SourceInfo
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableType {
    String,
    Int,
    Bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct  SourceInfo {
    pub line: i32,
    pub column: i32
}