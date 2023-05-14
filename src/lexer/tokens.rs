#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    DoubleColon,
    Ident,
    SemiColon,
    Mul,
    Div,
    Arrow,
    Plus,
    Min,
    Eq,
    Integer,
    Fn,
    Comma,
    LParen,   // TODO
    RParen,   // TODO
    Leq,      // TODO
    Geq,      // TODO
    DoubleEq, // TODO
    Neq,      // TODO
    Dot,      // TODO (will be used for function calls)
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

    pub fn get_prec(&self) -> i32 {
        match self.kind {
            Kind::Mul | Kind::Div => return 2,
            Kind::Plus | Kind::Min => return 1,
            _ => return 0,
        }
    }
}
