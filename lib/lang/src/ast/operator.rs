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
}

impl From<Operator> for ssf::ir::PrimitiveOperator {
    fn from(operator: Operator) -> Self {
        match operator {
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
        }
    }
}
