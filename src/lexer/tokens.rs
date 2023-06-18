#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Newline,
    DoubleColon,
    Ident,
    SemiColon,
    Mul,
    Div,
    Arrow,
    Plus,
    Min,
    Integer,
    String,
    Fn,
    Comma,
    LParen,
    RParen,
    If,
    QMark,
    Colon,
    Leq,
    Geq,
    DoubleEq,
    Neq,
    Use,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
}

impl Token {
    pub fn get_prec(&self) -> i32 {
        match self.kind {
            Kind::Mul | Kind::Div => return 3,
            Kind::Plus | Kind::Min => return 2,
            Kind::DoubleEq | Kind::Geq | Kind::Neq | Kind::Leq => return 1,
            _ => return 0,
        }
    }
}
