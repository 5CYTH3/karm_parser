use std::fmt::Display;

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

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match *self {
            Kind::Arrow => "=>",
            Kind::Colon => ":",
            Kind::Comma => ",",
            Kind::Div => "/",
            Kind::DoubleColon => "::",
            Kind::DoubleEq => "==",
            Kind::Fn => "fn",
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
            _ => "",
        };
        write!(f, "{}", data)
    }
}
