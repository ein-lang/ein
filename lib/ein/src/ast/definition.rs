use super::expression::Expression;
use super::function_definition::*;
use super::value_definition::*;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    ValueDefinition(ValueDefinition),
}

impl Definition {
    pub fn name(&self) -> &str {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition.name(),
            Self::ValueDefinition(value_definition) => value_definition.name(),
        }
    }

    pub fn type_(&self) -> &Type {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition.type_(),
            Self::ValueDefinition(value_definition) => value_definition.type_(),
        }
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.convert_expressions(convert).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.convert_expressions(convert).into()
            }
        }
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.convert_types(convert).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.convert_types(convert).into()
            }
        }
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition
                .resolve_reference_types(environment)
                .into(),
            Self::ValueDefinition(value_definition) => {
                value_definition.resolve_reference_types(environment).into()
            }
        }
    }
}

impl From<FunctionDefinition> for Definition {
    fn from(function_definition: FunctionDefinition) -> Self {
        Definition::FunctionDefinition(function_definition)
    }
}

impl From<ValueDefinition> for Definition {
    fn from(function_definition: ValueDefinition) -> Self {
        Definition::ValueDefinition(function_definition)
    }
}
