use super::expression::Expression;
use super::operator::Operator;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    operator: Operator,
    lhs: Rc<Expression>,
    rhs: Rc<Expression>,
}

impl Operation {
    pub fn new(operator: Operator, lhs: Expression, rhs: Expression) -> Self {
        Self {
            operator,
            lhs: Rc::new(lhs),
            rhs: Rc::new(rhs),
        }
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.operator,
            self.lhs.substitute_type_variables(substitutions),
            self.rhs.substitute_type_variables(substitutions),
        )
    }
}
