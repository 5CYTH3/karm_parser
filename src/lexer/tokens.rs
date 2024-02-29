use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
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
    Lam,
    Bar,
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

impl Debug for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match *self {
            Kind::Arrow => "=>",
            Kind::Colon => ":",
            Kind::Comma => ",",
            Kind::Div => "/",
            Kind::DoubleColon => "::",
            Kind::DoubleEq => "==",
            Kind::Lam => "lam",
            Kind::Geq => ">=",
            Kind::Ident => "IDENT",
            Kind::If => "if",
            Kind::Integer => "INT",
            Kind::LParen => "(",
            Kind::Leq => "<=",
            Kind::Min => "-",
            Kind::Mul => "*",
            Kind::Neq => "!=",
            Kind::Plus => "+",
            Kind::QMark => "?",
            Kind::RParen => ")",
            Kind::SemiColon => ";",
            Kind::String => "STR",
            Kind::Use => "USE",
            Kind::Bar => "|",
            _ => "",
        };
        write!(f, "{}", data)
    }
}
