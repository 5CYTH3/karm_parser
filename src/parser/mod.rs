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
            let exp = self.expr();
            program.push(exp.clone());
        }
        return program;
    }

    fn expr(&mut self) -> Expr {
        let next_token = match self.next.clone() {
            Some(token) => token,
            None => panic!("No lookahead."),
        };
        match next_token.kind {
            Kind::Fn => return self.fun_expr(),
            Kind::Integer => return self.numeric_expr(),
            _ => panic!(""),
        };
    }

    fn numeric_expr(&mut self) -> Expr {
        let data = self.eat(Kind::Integer);
        if self.next.clone().unwrap().is_op() {
            return self.binary_expr(data);
        }
        self.eat(Kind::SemiColon);
        Expr::Literal(data.value)
    }

    // PB: 5 * 8 + 9 should be evaluated as (5 * 8) + 9 but is evaluated as 5 * (8 + 9)
    fn binary_expr(&mut self, left: Token) -> Expr {
        let op = self.eat(self.next.clone().unwrap().kind);
        /*
         * The program will just wait for RHS to be self.expr() but LHS will always be integer.
         * But that means that the first integer coming will be LHS and then it does not take accountability of
         * the precedence of the current operator.
         */
        let right = self.expr();
        return Expr::Binary {
            op,
            lhs: Box::new(Expr::Literal(left.value)),
            rhs: Box::new(right),
        };
    }

    // ? Weird behaviour : Function Definitions can be nested.
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
