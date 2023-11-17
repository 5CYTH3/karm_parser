fn type_check_binary(&self, left: Type, right: Type, op: Kind) -> Result<Type, TypeError> {
        let expr_type = if left == right {
            left
        } else {
            return Err(TypeError(
                "Cannot compare two different types in a BinaryExpr".to_owned(),
            ));
        };

        let op_accepted_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => vec![Type::Int], 
            Kind::Neq | Kind::DoubleEq => vec![Type::Int, Type::Str, Type::Bool], 
            Kind::Geq | Kind::Leq => vec![Type::Int],
            _ => vec![Type::Invalid],
        };

        let op_match_type = match op {
            Kind::Mul | Kind::Div | Kind::Plus | Kind::Min => Type::Int,
            Kind::Neq | Kind::DoubleEq | Kind::Geq | Kind::Leq => Type::Bool,
            _ => Type::Invalid,
        };

        if !op_accepted_type.contains(&expr_type) {
            return Err(TypeError(
                "The lhs and rhs expressions cannot be compared with this operator.".to_owned(),
            ));
        }

        return Ok(op_match_type);
    }

    fn type_check_literal(&self, literal: &Literal) -> Type {
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