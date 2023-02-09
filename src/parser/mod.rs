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
        let next_token = self.next.clone().unwrap();
        match next_token.kind {
            Kind::Fn => {
                return self.fun_expr();
            }
            _ => return self.gen_expr(),
        };
    }

    fn fun_expr(&mut self) -> Expr {
        self.eat(Kind::Fn);
        let id = self.eat(Kind::Ident);
        self.eat(Kind::DoubleColon);
        let mut params: Vec<Token> = vec![];
        while self.next.clone().unwrap().kind != Kind::Arrow {
            if self.next.clone().unwrap().kind == Kind::Comma {
                self.eat(Kind::Comma);
            }
            params.push(self.eat(Kind::Ident));
            println!("{:?}", self.next);
        }
        self.eat(Kind::Arrow);
        return Expr::Fn {
            ident: id.value,
            params: Some(params),
            operation: Box::new(self.gen_expr()),
        };
    }

    // Might be preferable to just match the next token and if its operator -> return a binary_expr() function data.
    fn gen_expr(&mut self) -> Expr {
        println!("{:?}", self.next);

        let data = self.eat(Kind::Integer);
        if self.next.clone().unwrap().is_op() {
            let left = data;
            let op = self.eat(self.next.clone().unwrap().kind);
            /*
             * No more problem with recursion by having "self.expr()" as RHS assignment but now, we got a problem
             * with precedence, as the program will just wait for RHS to be self.expr() but LHS will always be integer.
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
        self.eat(Kind::SemiColon);
        Expr::Literal(data.value)
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
