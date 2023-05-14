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
    next: Option<Token>,
    lexer: Lexer,
}

impl Parser {
    pub fn new(program: String) -> Self {
        let mut lexer = Lexer::new(program);
        Self {
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
            program.push(match exp {
                Ok(val) => val,
                Err(e) => panic!("{}", e),
            });
        }
        program
    }

    fn expr_def(&mut self) -> Result<Expr, SyntaxError> {
        let expr = self.expr();
        match self.eat(Kind::SemiColon) {
            Ok(val) => (),
            Err(e) => return Err(e),
        };
        expr
    }

    fn expr(&mut self) -> Result<Expr, SyntaxError> {
        let next_token = match self.next.clone() {
            Some(token) => token,
            None => return Err(SyntaxError(vec![self.next.clone().unwrap().kind], None)),
        };
        let expr = match next_token.kind {
            Kind::Fn => self.fun_expr(),
            Kind::Integer => self.low_prec_expr(),
            Kind::Ident => self.function_call(),
            _ => {
                return Err(SyntaxError(
                    vec![Kind::Fn, Kind::Integer, Kind::Ident],
                    Some(next_token.kind),
                ))
            }
        };
        expr
    }

    fn literal(&mut self) -> Result<Expr, SyntaxError> {
        let literal = match self.next.clone().unwrap().kind {
            Kind::Integer => Expr::Literal(Literal::Int(match self.eat(Kind::Integer) {
                Ok(val) => val.value.to_string().parse::<i32>().unwrap(),
                Err(e) => return Err(e),
            })),
            _ => {
                return Err(SyntaxError(
                    vec![Kind::Integer, Kind::Ident],
                    Some(self.next.clone().unwrap().kind),
                ))
            }
        };
        Ok(literal)
    }

    fn function_call(&mut self) -> Result<Expr, SyntaxError> {
        let id = match self.eat(Kind::Ident) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        /*
        if self.next.unwrap().clone().kind != Kind::Dot {
            If next is not a dot => this is a variable. Else, this is a function call
        } */
        Ok(Expr::FnCall {
            ident: id.value,
            params: None,
        })
    }

    // Operation such as +, -
    fn low_prec_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = match self.high_prec_expr() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        while self.next.clone().unwrap().get_prec() == 1 {
            let op = match self.eat(self.next.clone().unwrap().kind) {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            let right = match self.high_prec_expr() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            left = Expr::Binary {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }
        Ok(left)
    }

    // Operation such as *, /
    fn high_prec_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = match self.literal() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        while self.next.clone().unwrap().get_prec() == 2 {
            let op = match self.eat(self.next.clone().unwrap().kind) {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            let right = match self.literal() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            left = Expr::Binary {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }
        Ok(left)
    }

    // ? Weird behaviour : Function Definitions can be nested (caused by the use of self.expr() without restriction).
    fn fun_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.eat(Kind::Fn);
        let id = match self.eat(Kind::Ident) {
            Ok(value) => value.value,
            Err(e) => return Err(e),
        };

        // Check if the function has parameters (if it has the :: operator, it has parameters).
        if self.next.clone().unwrap().kind == Kind::DoubleColon {
            let mut params: Vec<String> = vec![];
            self.eat(Kind::DoubleColon);
            while self.next.clone().unwrap().kind != Kind::Arrow {
                params.push(match self.eat(Kind::Ident) {
                    Ok(val) => val.value,
                    Err(e) => return Err(e),
                });
                if self.next.clone().unwrap().kind == Kind::Comma {
                    self.eat(Kind::Comma);
                }
            }
            self.eat(Kind::Arrow);
            return Ok(Expr::Fn {
                ident: id,
                params: Some(params),
                operation: Box::new(match self.expr() {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                }),
            });
        }

        // If the function has no parameters, return a Expr::Fn with `None` as params value.
        self.eat(Kind::Arrow);
        Ok(Expr::Fn {
            ident: id,
            params: None,
            operation: Box::new(match self.expr() {
                Ok(val) => val,
                Err(e) => return Err(e),
            }),
        })
    }

    fn eat(&mut self, kind_target: Kind) -> Result<Token, SyntaxError> {
        let t: Token = match &self.next {
            Some(val) => val.to_owned(),
            None => return Err(SyntaxError(vec![kind_target], None)),
        };

        let kind: Kind = t.clone().kind;

        if kind != kind_target {
            return Err(SyntaxError(vec![kind_target], Some(kind)));
        }

        let new_lookahead = self.lexer.get_next();
        self.next = new_lookahead;

        Ok(t)
    }
}
