use super::operation::Operation;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Operation(Operation),
    Variable(String),
}

impl From<f64> for Expression {
    fn from(number: f64) -> Expression {
        Expression::Number(number)
    }
}

impl From<Operation> for Expression {
    fn from(appication: Operation) -> Expression {
        Expression::Operation(appication)
    }
}
