#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<&Operator> for core::ast::Operator {
    fn from(operator: &Operator) -> Self {
        match operator {
            Operator::Add => core::ast::Operator::Add,
            Operator::Subtract => core::ast::Operator::Subtract,
            Operator::Multiply => core::ast::Operator::Multiply,
            Operator::Divide => core::ast::Operator::Divide,
        }
    }
}
