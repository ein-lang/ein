#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
}

impl From<Operator> for ssf::ir::PrimitiveOperator {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Equal => ssf::ir::PrimitiveOperator::Equal,
            Operator::NotEqual => ssf::ir::PrimitiveOperator::NotEqual,
        }
    }
}
