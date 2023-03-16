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
    Fn,
    Comma,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}

impl Token {
    pub fn is_op(&self) -> bool {
        return self.kind == Kind::Min
            || self.kind == Kind::Plus
            || self.kind == Kind::Div
            || self.kind == Kind::Mul;
    }
}
