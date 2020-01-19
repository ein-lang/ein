#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<&Operator> for ssf::ast::Operator {
    fn from(operator: &Operator) -> Self {
        match operator {
            Operator::Add => ssf::ast::Operator::Add,
            Operator::Subtract => ssf::ast::Operator::Subtract,
            Operator::Multiply => ssf::ast::Operator::Multiply,
            Operator::Divide => ssf::ast::Operator::Divide,
        }
    }
}
