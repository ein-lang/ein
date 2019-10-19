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

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition
                .substitute_type_variables(substitutions)
                .into(),
            Self::ValueDefinition(value_definition) => value_definition
                .substitute_type_variables(substitutions)
                .into(),
        }
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        let definition = match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.convert_definitions(convert).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.convert_definitions(convert).into()
            }
        };

        convert(&definition)
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
