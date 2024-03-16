use std::fmt::Debug;
use std::process::exit;

use crate::errors::SyntaxError;
use crate::lexer::tokens::{Kind, Token};
use crate::lexer::Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    LamCall {
        ident: String,
        style: LamStyle,
        params: Option<Vec<Expr>>,
    },
    LamDef {
        style: LamStyle,
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
    Use(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LamStyle {
    Infix,
    Prefix,
}

#[derive(Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Int(i32),
}

// TODO: Simple disclaimers :
// ! Could the parser potentially become an iterator too ?
// ! Should I use a more functional approach for the parser ?
#[derive(PartialEq)]
pub struct Program(pub Vec<Expr>);

pub struct Parser<'a> {
    next: Option<Token<'a>>,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a str) -> Self {
        let mut lexer = Lexer::new(program);
        Self {
            next: lexer.next(),
            lexer,
        }
    }

    pub fn program(mut self) -> Result<Program, SyntaxError> {
        if self.next.is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        self.parse()
    }

    pub fn parse(&mut self) -> Result<Program, SyntaxError> {
        let mut ast: Vec<Expr> = Vec::new();
        while self.next.is_some() {
            let exp = self.expr_def();
            ast.push(exp?);
        }
        Ok(Program(ast))
    }

    fn expr_def(&mut self) -> Result<Expr, SyntaxError> {
        let expr = self.expr();
        self.eat(&Kind::SemiColon)?;
        expr
    }

    fn expr(&mut self) -> Result<Expr, SyntaxError> {
        let next_token = match &self.next {
            Some(token) => token,
            None => {
                return Err(SyntaxError(
                    vec![self.next_token().kind],
                    None,
                    (self.lexer.col_cursor, self.lexer.line_cursor),
                ))
            }
        };

        match next_token.kind {
            Kind::Lam => self.lam_expr(),
            Kind::Use => self.use_expr(),
            _ => Err(SyntaxError(
                vec![Kind::Lam],
                Some(next_token.kind.to_owned()),
                (self.lexer.col_cursor, self.lexer.line_cursor),
            )),
        }
    }

    fn use_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.eat(&Kind::Use)?;
        let path = self.eat(&Kind::String)?.value;
        Ok(Expr::Use(path.to_string()))
    }

    // ? No more function nesting (we call if_exprs and not expr everywhere)
    fn lam_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.eat(&Kind::Lam)?;
        let id = self.eat(&Kind::Ident)?.value.to_string();
        let mut style = LamStyle::Prefix;
        if self.next_token().kind == Kind::Bar {
            self.eat(&Kind::Bar)?;
            style = LamStyle::Infix;
        }

        // Check if the function has parameters (if it has the :: operator, it has parameters).
        if self.next_token().kind == Kind::DoubleColon {
            let mut params: Vec<String> = vec![];
            self.eat(&Kind::DoubleColon)?;
            while self.next_token().kind != Kind::Arrow {
                params.push(self.eat(&Kind::Ident)?.value.to_string());
                if self.next_token().kind == Kind::Comma {
                    self.eat(&Kind::Comma)?;
                }
            }

            self.eat(&Kind::Arrow)?;
            return Ok(Expr::LamDef {
                ident: id.to_string(),
                style,
                params: Some(params),
                operation: Box::new(self.if_expr()?),
            });
        }

        // If the function has no parameters, return a Expr::Fn with `None` as params value.
        self.eat(&Kind::Arrow)?;

        Ok(Expr::LamDef {
            ident: id.to_string(),
            style,
            params: None,
            operation: Box::new(self.if_expr()?),
        })
    }

    fn if_expr(&mut self) -> Result<Expr, SyntaxError> {
        if self.next_token().kind == Kind::If {
            self.eat(&Kind::If)?;
            let mut cond: Expr = Expr::Literal(Literal::Int(0));
            let mut then: Expr = Expr::Literal(Literal::Int(0));
            let mut alter: Expr = Expr::Literal(Literal::Int(0));
            while self.next_token().kind != Kind::QMark {
                cond = self.binary_expr()?
            }
            self.eat(&Kind::QMark)?;
            while self.next_token().kind != Kind::Colon {
                then = self.binary_expr()?
            }
            self.eat(&Kind::Colon)?;
            while self.next_token().kind != Kind::SemiColon {
                alter = self.binary_expr()?
            }
            return Ok(Expr::If {
                cond: Box::from(cond),
                then: Box::from(then),
                alter: Box::from(alter),
            });
        }
        self.binary_expr()
    }

    fn binary_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.conditional_expr()
    }

    fn parenthesized_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.eat(&Kind::LParen)?;
        let expr = self.binary_expr();
        self.eat(&Kind::RParen)?;
        expr
    }

    fn conditional_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left: Expr = self.low_prec_expr()?;
        while self.next_token().get_prec() == 1 {
            let op = self.eat(&self.next_token().clone().kind)?.value.to_string();
            let right = self.low_prec_expr()?;
            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: Some(vec![left, right]),
            };
        }
        Ok(left)
    }

    // Operation such as +, - (expressions)
    fn low_prec_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left = self.high_prec_expr()?;
        while self.next_token().get_prec() == 2 {
            let op = match self.eat(&self.next_token().clone().kind) {
                Ok(val) => val.value.to_string(),
                Err(e) => return Err(e),
            };
            let right = self.high_prec_expr()?;
            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: Some(vec![left, right]),
            };
        }
        Ok(left)
    }

    // Operation such as *, /
    fn high_prec_expr(&mut self) -> Result<Expr, SyntaxError> {
        let mut left: Expr = self.factor()?;
        while self.next_token().get_prec() == 3 {
            let op = self.eat(&self.next_token().clone().kind)?.value.to_string();
            let right = self.factor()?;
            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: Some(vec![left, right]),
            };
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let literal: Result<Expr, SyntaxError> = match self.next_token().kind {
            Kind::Integer => Ok(Expr::Literal(Literal::Int(
                match self.eat(&Kind::Integer) {
                    Ok(val) => val.value.parse::<i32>().unwrap(),
                    Err(e) => return Err(e),
                },
            ))),
            Kind::String => Ok(Expr::Literal(Literal::Str(
                self.eat(&Kind::String)?.value.to_string(),
            ))),
            Kind::LParen => self.parenthesized_expr(),
            _ => self.ident(),
        };
        literal
    }

    fn ident(&mut self) -> Result<Expr, SyntaxError> {
        let id = self.eat(&Kind::Ident)?.value.to_string();
        if self.next_token().kind == Kind::LParen {
            let mut _params: Vec<Expr> = Vec::new();
            self.eat(&Kind::LParen)?;

            while self.next_token().kind != Kind::RParen {
                let param = self.conditional_expr()?;
                _params.push(param);
            }
            self.eat(&Kind::RParen)?;
            let params: Option<Vec<Expr>> = match _params.is_empty() {
                true => None,
                false => Some(_params),
            };
            return Ok(Expr::LamCall {
                ident: id,
                style: LamStyle::Prefix,
                params,
            });
        }
        Ok(Expr::Var(id))
    }

    fn next_token(&self) -> &Token {
        self.next.as_ref().unwrap()
    }

    fn eat(&mut self, kind_target: &Kind) -> Result<Token, SyntaxError> {
        let t: Token = match &self.next {
            Some(val) => val.to_owned(),
            None => {
                return Err(SyntaxError(
                    vec![*kind_target],
                    None,
                    (self.lexer.col_cursor, self.lexer.line_cursor),
                ))
            }
        };

        if &t.kind != kind_target {
            return Err(SyntaxError(
                vec![*kind_target],
                Some(t.kind),
                (self.lexer.col_cursor, self.lexer.line_cursor),
            ));
        }

        self.next = self.lexer.next();

        Ok(t)
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Str(string) => write!(f, "{}", string),
            Literal::Int(int) => write!(f, "{}", int),
        }
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .try_fold((), |_, expr| writeln!(f, "{:#?}", expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_func() {
        assert_eq!(
            Parser::new(r#"lam fib :: n -> if n <= 1 ? n : fib(n - 1) + fib(n - 2);"#)
                .program()
                .unwrap(),
            Program(vec![Expr::LamDef {
                ident: "fib".to_owned(),
                style: LamStyle::Prefix,
                params: Some(vec!["n".to_owned()]),
                operation: Box::from(Expr::If {
                    cond: Box::from(Expr::LamCall {
                        ident: "<=".to_owned(),
                        style: LamStyle::Infix,
                        params: Some(vec![
                            Expr::Var("n".to_owned()),
                            Expr::Literal(Literal::Int(1))
                        ])
                    }),
                    then: Box::from(Expr::Var("n".to_owned())),
                    alter: Box::from(Expr::LamCall {
                        ident: "+".to_owned(),
                        style: LamStyle::Infix,
                        params: Some(vec![
                            Expr::LamCall {
                                ident: "fib".to_owned(),
                                style: LamStyle::Prefix,
                                params: Some(vec![Expr::LamCall {
                                    ident: "-".to_owned(),
                                    style: LamStyle::Infix,
                                    params: Some(vec![
                                        Expr::Var("n".to_owned()),
                                        Expr::Literal(Literal::Int(1))
                                    ])
                                }])
                            },
                            Expr::LamCall {
                                ident: "fib".to_owned(),
                                style: LamStyle::Prefix,
                                params: Some(vec![Expr::LamCall {
                                    ident: "-".to_owned(),
                                    style: LamStyle::Infix,
                                    params: Some(vec![
                                        Expr::Var("n".to_owned()),
                                        Expr::Literal(Literal::Int(2))
                                    ])
                                }])
                            }
                        ]),
                    })
                })
            }])
        );
    }
}
