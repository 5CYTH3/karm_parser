#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    DoubleColon,
    Ident,
    SemiColon,
    Arrow,
    Plus,
    Mul,
    Div,
    Min,
    Eq,
    Integer,
    Let,
    Fun,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}
