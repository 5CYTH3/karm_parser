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
    Literal(Literal),
    FnCall {
        ident: String,
        params: Option<Box<Expr>>,
    },
    Fn {
        ident: String,
        params: Option<Vec<String>>,
        operation: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Int(i32),
}

type Program = Vec<Expr>;

pub struct Parser {
    program: String,
    next: Option<Token>,
    lexer: Lexer,
}

impl Parser {
    pub fn new(program: String) -> Self {
        let mut lexer = Lexer::new();
        lexer.init(program.clone());
        Self {
            program,
            next: lexer.get_next(),
            lexer,
        }
    }

    pub fn program(&mut self) -> Program {
        if self.next.is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        self.parse()
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.next.is_none() {
            let exp = self.expr_def();
            program.push(exp.clone());
        }
        program
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
            Kind::Ident => self.function_call(),
            _ => panic!("Invalid expr type."),
        };
        expr
    }

    fn literal(&mut self) -> Expr {
        match self.next.clone().unwrap().kind {
            Kind::Integer => Expr::Literal(Literal::Int(self.eat(Kind::Integer).value.to_string().parse::<i32>().unwrap())),
            Kind::Ident => Expr::FnCall { ident: self.eat(self.next.clone().unwrap().kind).value, params: Some(Box::new(self.expr())) }, // Instead of self.expr() we need to bring back self.arithmetic_expr()
            _ => panic!("")
        }
    }

    fn function_call(&mut self) -> Expr {
        let id = self.eat(Kind::Ident);
        Expr::FnCall {
            ident: id.value,
            params: None,
        }
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
            let mut params: Vec<String> = vec![];
            self.eat(Kind::DoubleColon);
            while self.next.clone().unwrap().kind != Kind::Arrow {
                params.push(self.eat(Kind::Ident).value);
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
        Expr::Fn {
            ident: id.value.,
            params: None,
            operation: Box::new(self.expr()),
        }
    }

    fn eat(&mut self, kind_target: Kind) -> Result<Token, SyntaxError> {
        let t: Token = match &self.next {
            Some(val) => val.to_owned(),
            None => return Err(SyntaxError(kind_target, None)),
        };

        let kind: Kind = t.clone().kind;

        if kind != kind_target {
            return Err(SyntaxError(kind_target, Some(kind)));
        }

        let new_lookahead = self.lexer.get_next();
        self.next = new_lookahead;

        Ok(t)
    }
}
