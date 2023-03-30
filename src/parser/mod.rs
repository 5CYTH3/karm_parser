use core::panic;
use std::process::exit;

use crate::errors::SyntaxError;
use crate::lexer::tokens::{Kind, Token};
use crate::lexer::Lexer;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Literal(String),
    Fn {
        ident: String,
        params: Option<Vec<Token>>,
        operation: Box<Expr>,
    },
}

type Program = Vec<Expr>;

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

    pub fn init(&mut self, program: String) -> Program {
        self.program = program.clone();
        self.lexer.init(program);
        self.next = self.lexer.get_next();
        return self.program();
    }

    fn program(&mut self) -> Program {
        if self.next.is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        return self.parse();
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.next.is_none() {
            let exp = self.expr_def();
            program.push(exp.clone());
        }
        return program;
    }

    fn expr_def(&mut self) -> Expr {
        let expr = self.expr();
        self.eat(Kind::SemiColon);
        expr
    }

    fn expr(&mut self) -> Expr {
        let next_token = match self.next.clone() {
            Some(token) => token,
            None => panic!("No lookahead."),
        };
        let expr = match next_token.kind {
            Kind::Fn => self.fun_expr(),
            Kind::Integer => self.low_prec_expr(),
            _ => panic!("Invalid expr type."),
        };
        expr
    }

    fn literal(&mut self) -> Expr {
        return Expr::Literal(self.eat(Kind::Integer).value);
    }

    // Operation such as +, -
    fn low_prec_expr(&mut self) -> Expr {
        let mut left = self.high_prec_expr();

        while self.next.clone().unwrap().get_prec() == 1 {
            let op = self.eat(self.next.clone().unwrap().kind);
            let right = self.high_prec_expr();
            left = Expr::Binary {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }
        left
    }

    // Operation such as *, /
    fn high_prec_expr(&mut self) -> Expr {
        let mut left: Expr = self.literal();
        while self.next.clone().unwrap().get_prec() == 2 {
            let op = self.eat(self.next.clone().unwrap().kind);
            let right = self.literal();
            left = Expr::Binary {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }
        left
    }

    // ? Weird behaviour : Function Definitions can be nested (caused by the use of self.expr() without restriction).
    fn fun_expr(&mut self) -> Expr {
        self.eat(Kind::Fn);
        let id = self.eat(Kind::Ident);

        // Check if the function has parameters (if it has the :: operator, it has parameters).
        if self.next.clone().unwrap().kind == Kind::DoubleColon {
            let mut params: Vec<Token> = vec![];
            self.eat(Kind::DoubleColon);
            while self.next.clone().unwrap().kind != Kind::Arrow {
                params.push(self.eat(Kind::Ident));
                if self.next.clone().unwrap().kind == Kind::Comma {
                    self.eat(Kind::Comma);
                }
            }
            self.eat(Kind::Arrow);
            return Expr::Fn {
                ident: id.value,
                params: Some(params),
                operation: Box::new(self.expr()),
            };
        }

        // If the function has no parameters, return a Expr::Fn with `None` as params value.
        self.eat(Kind::Arrow);
        return Expr::Fn {
            ident: id.value,
            params: None,
            operation: Box::new(self.expr()),
        };
    }

    fn eat(&mut self, kind_target: Kind) -> Token {
        let t: Token = match &self.next {
            Some(val) => val.to_owned(),
            None => panic!("{}", SyntaxError(kind_target, None)),
        };

        let kind: Kind = t.clone().kind;

        if kind != kind_target {
            panic!("{}", SyntaxError(kind_target, Some(kind)));
        }

        let new_lookahead = self.lexer.get_next();
        self.next = new_lookahead;

        t
    }
}
