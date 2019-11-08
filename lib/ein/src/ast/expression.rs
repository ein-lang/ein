use super::application::Application;
use super::definition::Definition;
use super::let_::Let;
use super::number::Number;
use super::operation::Operation;
use super::variable::Variable;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    Let(Let),
    Number(Number),
    Operation(Operation),
    Variable(Variable),
}

impl Expression {
    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Expression::Application(application) => {
                application.substitute_type_variables(substitutions).into()
            }
            Expression::Let(let_) => let_.substitute_type_variables(substitutions).into(),
            Expression::Number(number) => Expression::Number(number.clone()),
            Expression::Operation(operation) => {
                operation.substitute_type_variables(substitutions).into()
            }
            Expression::Variable(variable) => Expression::Variable(variable.clone()),
        }
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        match self {
            Self::Application(application) => application.convert_definitions(convert).into(),
            Self::Let(let_) => let_.convert_definitions(convert).into(),
            Self::Operation(operation) => operation.convert_definitions(convert).into(),
            _ => self.clone(),
        }
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        let expression = match self {
            Self::Application(application) => application.convert_expressions(convert).into(),
            Self::Let(let_) => let_.convert_expressions(convert).into(),
            Self::Operation(operation) => operation.convert_expressions(convert).into(),
            _ => self.clone(),
        };

        convert(&expression)
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

impl From<Number> for Expression {
    fn from(number: Number) -> Expression {
        Expression::Number(number)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Expression::Operation(operation)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Expression {
        Expression::Variable(variable)
    }
}
