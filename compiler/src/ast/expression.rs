use super::application::Application;
use super::let_::Let;
use super::operation::Operation;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    Let(Let),
    Number(f64),
    Operation(Operation),
    Variable(String),
}

impl Expression {
    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Expression::Application(application) => {
                application.substitute_type_variables(substitutions).into()
            }
            Expression::Let(let_) => let_.substitute_type_variables(substitutions).into(),
            Expression::Number(number) => Expression::Number(*number),
            Expression::Operation(operation) => {
                operation.substitute_type_variables(substitutions).into()
            }
            Expression::Variable(variable) => Expression::Variable(variable.clone()),
        }
    }
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
