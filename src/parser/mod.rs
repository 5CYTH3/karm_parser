use std::process::exit;

use crate::lexer::tokens::{Kind, Token};
use crate::lexer::Lexer;

#[derive(Debug)]
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
        {
            self.lexer.init(program.clone());
            self.program = program;
        }
        if self.lexer.get_next().is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        self.next = self.lexer.get_next();

        return self.parse();
    }

    pub fn parse(&mut self) -> ExprList {
        let mut program: ExprList = vec![];
        while !self.next.is_none() {
            program.push(self.expr())
        }
        return program;
    }

    fn expr(&mut self) -> Expr {
        println!("{:?}", self.next);
        let ret: Expr = match self.next.clone().unwrap().kind {
            Kind::Let => {
                self.eat(Kind::Let);
                let id = self.eat(Kind::Ident);
                self.eat(Kind::Eq);
                let val = self.eat(Kind::Integer);
                self.eat(Kind::SemiColon);
                Expr::Var {
                    ident: id,
                    value: Box::new(Expr::Unary(val)),
                }
            }
            _ => panic!("Under control (no)"),
        };
        ret
    }

    fn eat(&mut self, kind_target: Kind) -> Token {
        let t: Token = match self.next.clone() {
            Some(val) => val,
            None => panic!(
                "SyntaxError!    -> Expected: {:?} and got: None",
                kind_target
            ),
        };

        let kind: Kind = match t.clone() {
            Token { value: _, r#kind } => kind,
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
