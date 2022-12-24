use std::process::exit;

use crate::lexer::tokens::{Kind, Token};
use crate::lexer::Lexer;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary(Token),
    Fun(Box<Expr>),
    Var {
        ident: Token,
        value: Box<Expr>,
    },
}

type ExprList = Vec<Expr>;

pub struct Parser {
    program: String,
    next: Option<Token>,
    lexer: Lexer,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            program: String::from(""),
            next: Some(Token {
                kind: Kind::Integer,
                value: String::from("0"),
            }),
            lexer: Lexer::new(),
        }
    }

    pub fn init(&mut self, program: String) -> ExprList {
        self.program = program.clone();
        self.lexer.init(program);
        self.next = self.lexer.get_next();
        return self.program();
    }

    fn program(&mut self) -> ExprList {
        if self.next.is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        return self.parse();
    }

    pub fn parse(&mut self) -> ExprList {
        let mut program: ExprList = vec![];

        while !self.next.is_none() {
            let exp = self.expr();
            program.push(exp.clone());
        }
        return program;
    }

    fn expr(&mut self) -> Expr {
        let next_token = self.next.clone().unwrap();
        match next_token.kind {
            Kind::Let => {
                return self.var_expr();
            }
            _ => panic!("This is bad {:?}", next_token),
        };
    }

    fn var_expr(&mut self) -> Expr {
        self.eat(Kind::Let);
        let id = self.eat(Kind::Ident);
        self.eat(Kind::Eq);
        let val = self.eat(Kind::Integer);
        self.eat(Kind::SemiColon);
        return Expr::Var {
            ident: id,
            value: Box::new(Expr::Unary(val)),
        };
    }

    fn eat(&mut self, kind_target: Kind) -> Token {
        let t: Token = match &self.next {
            Some(val) => val.to_owned(),
            None => panic!(
                "SyntaxError!    -> Expected: {:?} and got: None",
                kind_target
            ),
        };

        let kind: Kind = match t.clone() {
            Token { value: _, kind } => kind,
        };

        if kind != kind_target {
            panic!(
                "UnexpectedToken!    -> Expected: {:?} and got: {:?}",
                kind, kind_target
            )
        }

        let new_lookahead = self.lexer.get_next();
        self.next = new_lookahead;

        return t;
    }
}
