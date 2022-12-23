mod tokens;
use tokens::Token;
enum Statement {
    Expr(Box<Statement>),
    Block(StatementList),
}

type StatementList = Vec<Statement>;

pub struct Lexer {
    data: StatementList,
    next: Statement,
}

pub enum Expr {
    Binary(Box<Expr>, Box<Expr>),
    Unary(Token),
}
