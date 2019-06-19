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
    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Definition::FunctionDefinition(function_definition) => function_definition
                .substitute_type_variables(substitutions)
                .into(),
            Definition::ValueDefinition(value_definition) => value_definition
                .substitute_type_variables(substitutions)
                .into(),
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
