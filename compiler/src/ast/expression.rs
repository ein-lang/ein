use super::application::Application;
use super::let_::Let;
use super::operation::Operation;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    Let(Let),
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

impl From<Let> for Expression {
    fn from(let_: Let) -> Expression {
        Expression::Let(let_)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Expression::Operation(operation)
    }
}
