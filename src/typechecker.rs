use core::hash::Hash;
use std::{process::exit, collections::BTreeSet};
use crate::errors::TypeError;

use crate::{
    lexer::tokens::Kind,
    parser::{Expr, Literal, Program},
};

pub enum Sig {
    // Used to represent function, A type Joined { i: Int, o: String } is equivalent to Int -> String
    Joined {
        i: Box<Sig>,
        o: Box<Sig>
    },
    // Classic unique type
    Unit(Type)
}

struct TypeScheme(Gamma, BTreeSet<Type>);

type Gamma = BTreeSet<Assumption>;

#[derive(PartialEq, Hash, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Type {
    Int,
    Str,
    Bool,
    Whatever,
    Invalid,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
struct Assumption {
    pub name: String,
    pub hypothesis: BTreeSet<Type>
}

impl Assumption {
    
}

pub struct TypeChecker {
    ast: Program,
}

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
            Expr::LamDef {
                ident: id,
                params: _,
                style: _,
                operation,
            } => self.type_check_function(id, &operation),
            Expr::Var(id) => self.type_check_args(id),
            Expr::Literal(l) => self.type_check_literal(l),
            Expr::If { cond, then, alter } => self.type_check_ifs(
                cond,
                then,
                alter,
            ),

            _ => Ok(TypeScheme(BTreeSet::new(), BTreeSet::new())),
        }
    }

    fn type_check_function(&self, ident: &String, body: &Expr) -> Result<TypeScheme, TypeError> {
        let TypeScheme(body_in_type, body_out_type) = self.type_check(body)?;
        println!("Args of the func: {:?}\n Return type: {:?}", body_in_type, body_out_type);
        Ok(TypeScheme(body_in_type, body_out_type))
    }

    fn type_check_args(&self, id: &String) -> Result<TypeScheme, TypeError> {
        Ok(TypeScheme(
            BTreeSet::from([
                Assumption { name: id.clone(), hypothesis: BTreeSet::from([Type::Int, Type::Str, Type::Bool]) }
            ]),
            BTreeSet::from([Type::Int, Type::Str, Type::Bool])
        ))
    }

    fn type_check_literal(&self, literal: &Literal) -> Result<TypeScheme, TypeError> {
        Ok(match literal {
            Literal::Int(_) => TypeScheme(BTreeSet::new(), BTreeSet::from([Type::Int])),
            Literal::Str(_) => TypeScheme(BTreeSet::new(), BTreeSet::from([Type::Str])),
        })
    }

    fn type_check_ifs(&self, cond_expr: &Expr, then_expr: &Expr, alter_expr: &Expr) -> Result<TypeScheme, TypeError> {
        let TypeScheme(cond_in_type, cond_out_type) = self.type_check(cond_expr)?;
        let TypeScheme(then_in_type, then_out_type) = self.type_check(then_expr)?;
        let TypeScheme(alter_in_type, alter_out_type) = self.type_check(alter_expr)?;
        
        if cond_out_type != BTreeSet::from([Type::Bool]) {
            return Err(TypeError(
                "Cannot use an expression that is not of type boolean as condition.".to_owned(),
            ));
        }

        if then_out_type != alter_out_type {
            return Err(TypeError("Cannot return two different types".to_owned()));
        }

        let intersected_in_types: BTreeSet<Assumption> = self.intersect_assumption_types(&cond_in_type, &then_in_type); 
        let twice_intersected_in_types: BTreeSet<Assumption> = self.intersect_assumption_types(&intersected_in_types, &alter_in_type);

        Ok(TypeScheme(twice_intersected_in_types, alter_out_type))
    }

    /* 
    fn intersect_assumption_types(&self, left: &BTreeSet<Assumption>, right: &BTreeSet<Assumption>) -> BTreeSet<Assumption> {
        let mut set = BTreeSet::new();    
        for i in left {
            println!("{:?}", left);
            for j in right {
                if i.hypothesis.is_empty()  {
                    return 
                }
                if i.name == j.name {
                    let hypothesis = i.hypothesis.intersection(&j.hypothesis).cloned().collect();
                    if hypothesis.is_empty() {
                        return Err(TypeError(""));
                    }
                    set.insert(Assumption { name: i.name.clone(), hypothesis });
                } else {
                    set.insert(i.clone());
                    set.insert(j.clone());
                }
            }
        }
        println!("Set: {:?}", set);
        set
    }
    */

}
