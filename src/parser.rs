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
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        alter: Box<Expr>,
    },
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

    pub fn program(mut self) -> Program {
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
                Err(e) => {
                    println!("{}", e);
                    exit(1)
                }
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
        let next_token = match &self.next {
            Some(token) => token,
            None => return Err(SyntaxError(vec![self.next.clone().unwrap().kind], None)),
        };
        let expr = match next_token.kind {
            Kind::Fn => self.fun_expr(),
            _ => Err(SyntaxError(
                vec![Kind::Fn],
                Some(next_token.kind.to_owned()),
            )),
        };
        expr
    }

    // ? No more function nesting (we call if_exprs and not expr everywhere)
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
                operation: Box::new(match self.if_expr() {
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
            operation: Box::new(match self.if_expr() {
                Ok(val) => val,
                Err(e) => return Err(e),
            }),
        })
    }

    fn if_expr(&mut self) -> Result<Expr, SyntaxError> {
        if self.next.clone().unwrap().kind == Kind::If {
            self.eat(Kind::If);
            let mut cond: Expr = Expr::Literal(Literal::Int(0));
            let mut then: Expr = Expr::Literal(Literal::Int(0));
            let mut alter: Expr = Expr::Literal(Literal::Int(0));
            while self.next.clone().unwrap().kind != Kind::QMark {
                /* A bit more complicated than that... we need a conditional statement */
                cond = match self.low_prec_expr() {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                };
            }
            self.eat(Kind::QMark);
            while self.next.clone().unwrap().kind != Kind::Colon {
                then = match self.low_prec_expr() {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                }
            }
            self.eat(Kind::Colon);
            while self.next.clone().unwrap().kind != Kind::SemiColon {
                alter = match self.low_prec_expr() {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                }
            }
            return Ok(Expr::If {
                cond: Box::from(cond),
                then: Box::from(then),
                alter: Box::from(alter),
            });
        }
        self.low_prec_expr()
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
        let mut left: Expr = match self.conditional_expr() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        while self.next.clone().unwrap().get_prec() == 2 {
            let op = match self.eat(self.next.clone().unwrap().kind) {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            let right = match self.conditional_expr() {
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

    // TODO: Binary expressions with conditional expressions are allowed. Maybe put it on top instead of here so binary expressions work the right way.
    fn conditional_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left: Expr = match self.factor() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };
        while self.next.clone().unwrap().get_prec() == 3 {
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
            // TODO: Params seems not to work.
            Kind::LParen => {
                let mut _params: Vec<Expr> = Vec::new();
                self.eat(Kind::LParen);
                while self.next.clone().unwrap().kind != Kind::RParen {
                    let param = match self.low_prec_expr() {
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
    fn bin_id_expr() {
        assert_eq!(
            Parser::new(r#"fn main :: n -> n + 3;"#.to_owned()).program(),
            [Expr::Fn {
                ident: "main".to_owned(),
                params: Some(vec!["n".to_owned()]),
                operation: Box::from(Expr::Binary { 
                    op: Token { kind: Kind::Plus, value: "+".to_owned() }, 
                    lhs: Box::from(Expr::FnCall {
                        ident: "n".to_owned(),
                        params: None
                    }),
                    rhs: Box::from(Expr::Literal(Literal::Int(3))) 
                })
            }]
        );
    }

    #[test]
    fn if_expr() {
        assert_eq!(
            Parser::new(r#"fn main :: n -> if n <= 1 ? n : 0;"#.to_owned()).program(),
            [Expr::Fn {
                ident: "main".to_owned(),
                params: Some(vec!["n".to_owned()]),
                operation: Box::from(
                    Expr::If {
                        cond: Box::from(Expr::Binary {
                            op: Token { kind: Kind::Leq, value: "<=".to_owned() },
                            lhs: Box::from(Expr::FnCall { ident: "n".to_owned(), params: None }),
                            rhs: Box::from(Expr::Literal(Literal::Int(1)))
                        }),
                        then: Box::from(
                            Expr::FnCall { ident: "n".to_owned(), params: None }
                        ),
                        alter: Box::from(Expr::Literal(Literal::Int(0)))
                    }
                )
            }]
        );
    }
}
