pub enum Token {
    Operator(Operator),
    String(String),
    Integer(i32),
    Comma,
    Identifier(String),
    EOF,
}

pub enum Operator {
    Plus,
    Min,
    Div,
    Mul,
    Assign,
}
