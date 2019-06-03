use super::expression::Expression;
use super::operator::Operator;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    operator: Operator,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl Application {
    pub fn new(operator: Operator, lhs: Expression, rhs: Expression) -> Self {
        Application {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
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
}
