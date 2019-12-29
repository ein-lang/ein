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
            Self::Application(application) => {
                application.substitute_type_variables(substitutions).into()
            }
            Self::Let(let_) => let_.substitute_type_variables(substitutions).into(),
            Self::Number(number) => Self::Number(number.clone()),
            Self::Operation(operation) => operation.substitute_type_variables(substitutions).into(),
            Self::Variable(variable) => Self::Variable(variable.clone()),
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

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        match self {
            Self::Application(application) => {
                application.resolve_reference_types(environment).into()
            }
            Self::Let(let_) => let_.resolve_reference_types(environment).into(),
            Self::Operation(operation) => operation.resolve_reference_types(environment).into(),
            _ => self.clone(),
        }
    }
}

impl From<Application> for Expression {
    fn from(application: Application) -> Expression {
        Self::Application(application)
    }
}

impl From<Let> for Expression {
    fn from(let_: Let) -> Expression {
        Self::Let(let_)
    }
}

impl From<Number> for Expression {
    fn from(number: Number) -> Expression {
        Self::Number(number)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Self::Operation(operation)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Expression {
        Self::Variable(variable)
    }
}
