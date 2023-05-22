use core::panic;
use std::process::exit;

use crate::errors::SyntaxError;
use crate::lexer::tokens::{Kind, Token};
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Literal(Literal),
    FnCall {
        ident: String,
        params: Option<Vec<Expr>>,
    },
    Fn {
        ident: String,
        params: Option<Vec<String>>,
        operation: Box<Expr>,
    },
    Var(String),
}

#[derive(Debug, Clone, PartialEq)]
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
            _ => self.low_prec_expr(),
        };
        expr
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let literal: Result<Expr, SyntaxError> = match self.next.clone().unwrap().kind {
            Kind::Integer => Ok(Expr::Literal(Literal::Int(match self.eat(Kind::Integer) {
                Ok(val) => val.value.to_string().parse::<i32>().unwrap(),
                Err(e) => return Err(e),
            }))),
            Kind::String => Ok(Expr::Literal(Literal::Str(match self.eat(Kind::String) {
                Ok(val) => val.value,
                Err(e) => return Err(e),
            }))),
            _ => self.ident(),
        };
        literal
    }

    fn ident(&mut self) -> Result<Expr, SyntaxError> {
        let params: Option<Vec<Expr>> = match self.next.clone().unwrap().kind {
            Kind::LParen => {
                let mut _params: Vec<Expr> = Vec::new();
                self.eat(Kind::LParen);
                while self.next.clone().unwrap().kind != Kind::RParen {
                    let param = match self.expr() {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };
                    _params.push(param);
                }
                self.eat(Kind::RParen);
                Some(_params)
            }
            _ => None,
        };
        let id = match self.eat(Kind::Ident) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        Ok(Expr::FnCall {
            ident: id.value,
            params: params,
        })
    }

    // Operation such as +, - (expressions)
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
        let mut left = match self.factor() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        while self.next.clone().unwrap().get_prec() == 2 {
            let op = match self.eat(self.next.clone().unwrap().kind) {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            let right = match self.factor() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_expr_int() {
        assert_eq!(
            Parser::new("3 + 4;".to_owned()).program(),
            [Expr::Binary {
                op: Token {
                    kind: Kind::Plus,
                    value: "+".to_owned()
                },
                lhs: Box::from(Expr::Literal(Literal::Int(3))),
                rhs: Box::from(Expr::Literal(Literal::Int(4)))
            }]
        );
    }

    #[test]
    fn binary_expr_str() {
        assert_eq!(
            Parser::new(r##""Helloworld" + 4;"##.to_owned()).program(),
            [Expr::Binary {
                op: Token {
                    kind: Kind::Plus,
                    value: "+".to_owned()
                },
                lhs: Box::from(Expr::Literal(Literal::Str(r#""Helloworld""#.to_owned()))),
                rhs: Box::from(Expr::Literal(Literal::Int(4)))
            }]
        );
    }

    #[test]
    fn function_expr() {
        assert_eq!(
            Parser::new(r#"fn main :: n -> n;"#.to_owned()).program(),
            [Expr::Fn {
                ident: "main".to_owned(),
                params: Some(vec!["n".to_owned()]),
                operation: Box::from(Expr::FnCall {
                    ident: "n".to_owned(),
                    params: None
                })
            }]
        );
    }

    #[test]
    fn function_in_function_expr() {
        assert_eq!(
            Parser::new(r#"fn main :: n -> fn help -> n;"#.to_owned()).program(),
            [Expr::Fn {
                ident: "main".to_owned(),
                params: Some(vec!["n".to_owned()]),
                operation: Box::from(Expr::Fn {
                    ident: "help".to_owned(),
                    params: None,
                    operation: Box::from(Expr::FnCall {
                        ident: "n".to_owned(),
                        params: None
                    })
                })
            }]
        );
    }
}
