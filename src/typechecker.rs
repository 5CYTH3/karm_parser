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
        let lhs = left?;
        let rhs = right?;

        let expr_type = if lhs == rhs {
            lhs
        } else {
            return Err(TypeError(
                "Cannot compare two different types in a BinaryExpr".to_owned(),
            ));
        };

        // Little confusion here. This is not the type of the whole expression but the types that are accepted on the left and right
        // So second line should be allowed for almost all types
        let op_expected_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => vec![Type::Int],
            Kind::Neq | Kind::DoubleEq => vec![Type::Int, Type::Str, Type::Bool],
            Kind::Geq | Kind::Leq => vec![Type::Int],
            _ => vec![Type::Invalid],
        };

        if !op_expected_type.contains(&expr_type) {
            return Err(TypeError(
                "The left and right expressions does cannot be compared with this operator."
                    .to_owned(),
            ));
        }

        return Ok(expr_type);
    }

    fn type_check_literal(&self, literal: &Literal) -> Type {
        match literal {
            Literal::Int(i) => Type::Int,
            Literal::Str(s) => Type::Str,
        }
    }
}
