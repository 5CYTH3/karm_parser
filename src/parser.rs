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
        params: Vec<Expr>,
    },
    LamDef {
        style: LamStyle,
        ident: String,
        params: Vec<String>,
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
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a str) -> Self {
        let lexer = Lexer::new(program);
        Self {
            lexer,
        }
    }

    pub fn program(mut self) -> Result<Program, SyntaxError> {
        if self.peek().is_none() {
            println!("Program Terminated : Lookahead is empty, nothing to parse.");
            exit(1)
        }
        self.parse()
    }

    pub fn parse(&mut self) -> Result<Program, SyntaxError> {
        let mut ast: Vec<Expr> = Vec::new();
        while self.peek().is_some() {
            let exp = self.expr_def();
            ast.push(exp?);
        }
        Ok(Program(ast))
    }

    fn expr_def(&mut self) -> Result<Expr, SyntaxError> {
        let expr = self.expr();
        self.next(&Kind::SemiColon)?;
        expr
    }

    fn expr(&mut self) -> Result<Expr, SyntaxError> {
        let next_token = match self.peek() {
            Some(token) => token,
            None => {
                return Err(SyntaxError(
                    vec![self.peek().unwrap().kind],
                    None,
                    self.lexer.coords
                ))
            }
        };

        match next_token.kind {
            Kind::Lam => self.lam_expr(),
            Kind::Use => self.use_expr(),
            _ => Err(SyntaxError(
                vec![Kind::Lam],
                Some(next_token.kind),
                self.lexer.coords
            )),
        }
    }

    fn use_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.next(&Kind::Use)?;
        let path = self.next(&Kind::String)?.value;
        Ok(Expr::Use(path.to_string()))
    }

    // ? No more function nesting (we call if_exprs and not expr everywhere)
    fn lam_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.next(&Kind::Lam)?;

        // Represent the parsed parameters identifiers
        let mut params: Vec<String> = Vec::new();

        // Identifier of the function
        let id = self.next(&Kind::Ident)?.value.to_string();

        // Prefix | Infix 
        let mut style = LamStyle::Prefix;

        let next_token = self.peek().unwrap();

        if next_token.kind == Kind::Bar {
            self.next(&Kind::Bar)?;
            style = LamStyle::Infix;
        }

        // Check if the function has parameters (if it has the :: operator, it has parameters).
        if next_token.kind == Kind::DoubleColon {
            self.next(&Kind::DoubleColon)?;
            while self.peek().unwrap().kind != Kind::Arrow {
                params.push(self.next(&Kind::Ident)?.value.to_string());
                if self.peek().unwrap().kind == Kind::Comma {
                    self.next(&Kind::Comma)?;
                }
            }
        }

        self.next(&Kind::Arrow)?;

        Ok(Expr::LamDef {
            ident: id.to_string(),
            style,
            params,
            operation: Box::new(self.if_expr()?),
        })
    }

    fn if_expr(&mut self) -> Result<Expr, SyntaxError> {
        
        let next_token = self.peek().unwrap();

        if next_token.kind == Kind::If {

            self.next(&Kind::If)?;

            let cond: Expr = self.if_expr()?;
            self.next(&Kind::QMark)?;

            let then: Expr = self.if_expr()?;
            self.next(&Kind::Colon)?;

            let alter: Expr = self.if_expr()?;

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
        self.next(&Kind::LParen)?;
        let expr = self.if_expr();
        self.next(&Kind::RParen)?;
        expr
    }

    fn conditional_expr(&mut self) -> Result<Expr, SyntaxError> {

        let mut left: Expr = self.low_prec_expr()?;

        let next_token =  self.peek().unwrap();

        while next_token.get_prec() == 1 {

            let op = self.next(&next_token.kind)?.value.to_string();

            let right = self.low_prec_expr()?;

            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: vec![left, right],
            };

        }

        Ok(left)
    }

    // Operation such as +, - (expressions)
    fn low_prec_expr(&mut self) -> Result<Expr, SyntaxError> {

        let mut left = self.high_prec_expr()?;

        let next_token = self.peek().unwrap();

        while next_token.get_prec() == 2 {

            let op = match self.next(&next_token.kind) {
                Ok(val) => val.value.to_string(),
                Err(e) => return Err(e),
            };
            
            let right = self.high_prec_expr()?;

            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: vec![left, right],
            };

        }

        Ok(left)
    }

    // Operation such as *, /
    fn high_prec_expr(&mut self) -> Result<Expr, SyntaxError> {

        let mut left: Expr = self.factor()?;

        let next_token = self.peek().unwrap();

        while next_token.get_prec() == 3 {

            let op = self.next(&next_token.kind)?.value.to_string();

            let right = self.factor()?;

            left = Expr::LamCall {
                ident: op.to_string(),
                style: LamStyle::Infix,
                params: vec![left, right],
            };

        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        match self.peek().unwrap().kind {
            Kind::Integer => Ok(Expr::Literal(Literal::Int(
                match self.next(&Kind::Integer) {
                    Ok(val) => val.value.parse::<i32>().unwrap(),
                    Err(e) => return Err(e),
                },
            ))),
            Kind::String => Ok(Expr::Literal(Literal::Str(
                self.next(&Kind::String)?.value.to_string(),
            ))),
            Kind::LParen => self.parenthesized_expr(),
            _ => self.ident(),
        }
    }

    fn ident(&mut self) -> Result<Expr, SyntaxError> {
        let id = self.next(&Kind::Ident)?.value.to_string();

        if self.peek().unwrap().kind == Kind::LParen {
            let mut params: Vec<Expr> = Vec::new();
            self.next(&Kind::LParen)?;

            while self.peek().unwrap().kind != Kind::RParen {
                let param = self.conditional_expr()?;
                params.push(param);
            }
            self.next(&Kind::RParen)?;

            return Ok(Expr::LamCall {
                ident: id,
                style: LamStyle::Prefix,
                params,
            });
        }

        Ok(Expr::Var(id))
    }

    fn peek(&self) -> Option<&Token> {
        self.lexer.peekable().peek()
    }

    fn next(&mut self, kind_target: &Kind) -> Result<Token, SyntaxError> {

        let t: Token = match self.peek() {
            Some(val) => *val,

            // The sequence does not match any token
            None => {
                return Err(SyntaxError(
                    vec![*kind_target],
                    None,
                    self.lexer.coords
                ))
            }
        };

        if &t.kind != kind_target {
            return Err(SyntaxError(
                vec![*kind_target],
                Some(t.kind),
                self.lexer.coords
            ));
        }

        self.lexer.next();

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
                params: vec!["n".to_owned()],
                operation: Box::from(Expr::If {
                    cond: Box::from(Expr::LamCall {
                        ident: "<=".to_owned(),
                        style: LamStyle::Infix,
                        params: vec![
                            Expr::Var("n".to_owned()),
                            Expr::Literal(Literal::Int(1))
                        ]
                    }),
                    then: Box::from(Expr::Var("n".to_owned())),
                    alter: Box::from(Expr::LamCall {
                        ident: "+".to_owned(),
                        style: LamStyle::Infix,
                        params: vec![
                            Expr::LamCall {
                                ident: "fib".to_owned(),
                                style: LamStyle::Prefix,
                                params: vec![Expr::LamCall {
                                    ident: "-".to_owned(),
                                    style: LamStyle::Infix,
                                    params: vec![
                                        Expr::Var("n".to_owned()),
                                        Expr::Literal(Literal::Int(1))
                                    ]
                                }]
                            },
                            Expr::LamCall {
                                ident: "fib".to_owned(),
                                style: LamStyle::Prefix,
                                params: vec![Expr::LamCall {
                                    ident: "-".to_owned(),
                                    style: LamStyle::Infix,
                                    params: vec![
                                        Expr::Var("n".to_owned()),
                                        Expr::Literal(Literal::Int(2))
                                    ]
                                }]
                            }
                        ],
                    })
                })
            }])
        );
    }
}
