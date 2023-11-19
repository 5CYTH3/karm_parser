use core::hash::Hash;
use std::{process::exit, collections::BTreeSet};


use crate::{
    lexer::tokens::Kind,
    parser::{Expr, Literal, Program},
};

struct TypeScheme(Gamma, BTreeSet<Type>);

type Gamma = BTreeSet<Assumption>;

#[derive(PartialEq, Hash, Eq, PartialOrd, Ord, Clone)]
pub enum Type {
    Int,
    Str,
    Bool,
    Whatever,
    Invalid,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Assumption {
    name: String,
    hypothesis: BTreeSet<Type>
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
        for expr in &self.ast.0 {
            match self.type_check(expr) {
                Ok(_) => continue,
                Err(e) => {
                    println!("{:?}", e);
                    exit(1);
                }
            }
        }
    }

    fn type_check(&self, expr: &Expr) -> Result<TypeScheme, TypeError> {
        match expr {
            Expr::Fn {
                ident: _,
                params: _,
                operation,
            } => self.type_check(&operation),
            Expr::Binary { op, lhs, rhs } => self.type_check_binary(lhs, rhs, op),
            Expr::Var(id) => Ok(self.type_check_args(id)),
            Expr::Literal(l) => Ok(self.type_check_literal(l)),
            Expr::If { cond, then, alter } => self.type_check_ifs(
                self.type_check(cond)?,
                self.type_check(then)?,
                self.type_check(alter)?,
            ),
            
            _ => Ok(Type::Whatever),
        }
    }

    fn type_check_binary(&self, left: &Expr, right: &Expr, op: &Kind) -> Result<TypeScheme, TypeError> {
        let TypeScheme(t_left_in, t_left_out) = self.type_check(left)?;
        let TypeScheme(t_right_in, t_right_out) = self.type_check(right)?;

        // Get the possible types for the output of the function
        let intersected_t_expr: BTreeSet<Assumption> = t_left_in.intersection(&t_right_in).cloned().collect();

        // Check if there is no common possible types between the two expressions
        let t_expr = if !intersected_t_expr.is_empty() {
            intersected_t_expr
        } else {
            return Err(TypeError(
                "Cannot compare two different types in a BinaryExpr".to_owned(),
            ));
        };

        // The types accepted by each ops
        let op_accepted_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => BTreeSet::from([Type::Int]), 
            Kind::Neq | Kind::DoubleEq => BTreeSet::from([Type::Int, Type::Str, Type::Bool]), 
            Kind::Geq | Kind::Leq => BTreeSet::from([Type::Int]),
            _ => BTreeSet::from([Type::Invalid]),
        };

        // The type of the expression based on the op type
        let op_match_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => Type::Int,
            Kind::Neq | Kind::DoubleEq | Kind::Geq | Kind::Leq => Type::Bool,
            _ => Type::Invalid,
        };

        if !op_accepted_type.is_superset(&t_expr) {
            return Err(TypeError(
                "The lhs and rhs expressions cannot be compared with this operator.".to_owned(),
            ));
        }

        let expr_in: BTreeSet<Assumption> = t_left_in.union(&t_right_in).cloned().collect();

        return Ok(TypeScheme(expr_in, BTreeSet::from([op_match_type])));
    }

    fn type_check_args(&self, id: &String) -> Assumption {
        Assumption { name: *id, hypothesis: BTreeSet::from([Type::Int, Type::Str, Type::Bool]) }
    }

    fn type_check_literal(&self, literal: &Literal) -> TypeScheme {
        match literal {
            Literal::Int(_) => Type::Int,
            Literal::Str(_) => Type::Str,
        }
    }

    fn type_check_ifs(&self, cond_type: Type, then: Type, alter: Type) -> Result<Type, TypeError> {
        if cond_type != Type::Bool {
            return Err(TypeError(
                "Cannot use an expression that is not of type boolean as condition.".to_owned(),
            ));
        }

        if then != alter {
            return Err(TypeError("Cannot return two different types".to_owned()));
        }

        Ok(then)
    }

}
