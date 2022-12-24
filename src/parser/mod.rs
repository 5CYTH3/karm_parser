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
    Fun {
        ident: Token,
        params: Vec<Token>,

        operation: Box<Expr>,
    },
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
            Kind::Fun => {
                return self.fun_expr();
            }
            _ => panic!("This is bad {:?}", next_token),
        };
    }

    fn fun_expr(&mut self) -> Expr {
        self.eat(Kind::Fun);

        let id = self.eat(Kind::Ident);

        self.eat(Kind::DoubleColon);

        let mut params: Vec<Token> = vec![];
        while self.next.clone().unwrap().kind != Kind::Arrow {
            if self.next.clone().unwrap().kind == Kind::Comma {
                self.eat(Kind::Comma);
            } else {
            }
            params.push(self.eat(Kind::Ident));
            println!("{:?}", self.next);
        }
        self.eat(Kind::Arrow);
        return Expr::Fun {
            ident: id,
            params,
            operation: Box::new(self.gen_expr()),
        };
    }

    fn gen_expr(&mut self) -> Expr {
        println!("{:?}", self.next);

        let data = self.eat(Kind::Integer);
        if self.next.clone().unwrap().is_op() {
            let left = data;
            println!("{:?}", self.next);

            let op = self.eat(self.next.clone().unwrap().kind);
            // PROBLEM. (Recursion). We cannot know if the RHS expr will be a Binary or Unary expr.
            println!("{:?}", self.next);

            let right = self.eat(Kind::Integer);
            return Expr::Binary {
                op,
                lhs: Box::new(Expr::Unary(left)),
                rhs: Box::new(Expr::Unary(right)),
            };
        }
        self.eat(Kind::SemiColon);
        Expr::Unary(data)
    }

    fn var_expr(&mut self) -> Expr {
        self.eat(Kind::Let);
        let id = self.eat(Kind::Ident);
        self.eat(Kind::Eq);
        let val = self.gen_expr();

        return Expr::Var {
            ident: id,
            value: Box::new(val),
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
