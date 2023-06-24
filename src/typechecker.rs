use std::process::exit;

use crate::{
    lexer::tokens::Kind,
    parser::{Expr, Literal, Program},
};

#[derive(PartialEq)]
pub enum Type {
    Int,
    Str,
    Bool,
    Whatever,
    Invalid,
}

pub struct TypeChecker {
    ast: Program,
}

#[derive(Debug)]
struct TypeError(String);

impl TypeChecker {
    pub fn new(ast: Program) -> Self {
        TypeChecker { ast }
    }

    pub fn init(&self) {
        for expr in &self.ast {
            match self.type_check(expr) {
                Ok(t) => continue,
                Err(e) => {
                    println!("{:?}", e);
                    exit(1);
                }
            }
        }
    }

    fn type_check(&self, expr: &Expr) -> Result<Type, TypeError> {
        match expr {
            Expr::Fn {
                ident,
                params,
                operation,
            } => self.type_check(&operation),
            Expr::Binary { op, lhs, rhs } => {
                self.type_check_binary(self.type_check(lhs), self.type_check(rhs), *op)
            }
            Expr::Literal(l) => Ok(self.type_check_literal(l)),
            _ => Ok(Type::Whatever),
        }
    }

    fn type_check_binary(
        &self,
        left: Result<Type, TypeError>,
        right: Result<Type, TypeError>,
        op: Kind,
    ) -> Result<Type, TypeError> {
        let lhs = match left {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let rhs = match right {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let expr_type = if lhs == rhs {
            lhs
        } else {
            return Err(TypeError(
                "Cannot compare two different types in a BinaryExpr".to_owned(),
            ));
        };

        let op_expected_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => Type::Int,
            Kind::DoubleEq | Kind::Geq | Kind::Neq | Kind::Leq => Type::Bool,
            _ => Type::Invalid,
        };

        if expr_type != op_expected_type {
            return Err(TypeError(
                "The left and right expressions does cannot be compared with this operator."
                    .to_owned(),
            ));
        }

        return Ok(op_expected_type);
    }

    fn type_check_literal(&self, literal: &Literal) -> Type {
        match literal {
            Literal::Int(i) => Type::Int,
            Literal::Str(s) => Type::Str,
        }
    }
}
