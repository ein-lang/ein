#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl From<Operator> for ssf::ir::PrimitiveOperator {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Equal => ssf::ir::PrimitiveOperator::Equal,
            Operator::NotEqual => ssf::ir::PrimitiveOperator::NotEqual,
            Operator::LessThan => ssf::ir::PrimitiveOperator::LessThan,
            Operator::LessThanOrEqual => ssf::ir::PrimitiveOperator::LessThanOrEqual,
            Operator::GreaterThan => ssf::ir::PrimitiveOperator::GreaterThan,
            Operator::GreaterThanOrEqual => ssf::ir::PrimitiveOperator::GreaterThanOrEqual,
        }
    }
}
