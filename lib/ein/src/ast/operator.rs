use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

impl TryFrom<Operator> for ssf::ir::PrimitiveOperator {
    type Error = ();

    fn try_from(operator: Operator) -> Result<Self, ()> {
        Ok(match operator {
            Operator::Add => ssf::ir::PrimitiveOperator::Add,
            Operator::Subtract => ssf::ir::PrimitiveOperator::Subtract,
            Operator::Multiply => ssf::ir::PrimitiveOperator::Multiply,
            Operator::Divide => ssf::ir::PrimitiveOperator::Divide,
            Operator::Equal => ssf::ir::PrimitiveOperator::Equal,
            Operator::NotEqual => ssf::ir::PrimitiveOperator::NotEqual,
            Operator::LessThan => ssf::ir::PrimitiveOperator::LessThan,
            Operator::LessThanOrEqual => ssf::ir::PrimitiveOperator::LessThanOrEqual,
            Operator::GreaterThan => ssf::ir::PrimitiveOperator::GreaterThan,
            Operator::GreaterThanOrEqual => ssf::ir::PrimitiveOperator::GreaterThanOrEqual,
            Operator::And | Operator::Or => return Err(()),
        })
    }
}
