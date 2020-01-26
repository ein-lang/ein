#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<&Operator> for ssf::ir::Operator {
    fn from(operator: &Operator) -> Self {
        match operator {
            Operator::Add => ssf::ir::Operator::Add,
            Operator::Subtract => ssf::ir::Operator::Subtract,
            Operator::Multiply => ssf::ir::Operator::Multiply,
            Operator::Divide => ssf::ir::Operator::Divide,
        }
    }
}
