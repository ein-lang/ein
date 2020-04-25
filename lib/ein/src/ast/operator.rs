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

impl From<Operator> for ssf::ir::Operator {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Add => ssf::ir::Operator::Add,
            Operator::Subtract => ssf::ir::Operator::Subtract,
            Operator::Multiply => ssf::ir::Operator::Multiply,
            Operator::Divide => ssf::ir::Operator::Divide,
            Operator::Equal => ssf::ir::Operator::Equal,
            Operator::NotEqual => ssf::ir::Operator::NotEqual,
            Operator::LessThan => ssf::ir::Operator::LessThan,
            Operator::LessThanOrEqual => ssf::ir::Operator::LessThanOrEqual,
            Operator::GreaterThan => ssf::ir::Operator::GreaterThan,
            Operator::GreaterThanOrEqual => ssf::ir::Operator::GreaterThanOrEqual,
        }
    }
}
