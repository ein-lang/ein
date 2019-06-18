use super::application::Application;
use super::let_values::LetValues;
use super::operation::Operation;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    LetValues(LetValues),
    Number(f64),
    Operation(Operation),
    Variable(String),
}

impl From<f64> for Expression {
    fn from(number: f64) -> Expression {
        Expression::Number(number)
    }
}

impl From<Application> for Expression {
    fn from(application: Application) -> Expression {
        Expression::Application(application)
    }
}

impl From<LetValues> for Expression {
    fn from(let_values: LetValues) -> Expression {
        Expression::LetValues(let_values)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Expression::Operation(operation)
    }
}
