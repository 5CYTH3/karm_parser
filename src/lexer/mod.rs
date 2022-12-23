mod tokens;
use tokens::Token;

use self::tokens::Operator;
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
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary(Token),
}
