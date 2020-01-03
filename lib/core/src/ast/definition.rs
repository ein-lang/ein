use super::function_definition::*;
use super::value_definition::*;
use crate::types::Type;
use std::collections::{HashMap, HashSet};

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

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.rename_variables(names).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.rename_variables(names).into()
            }
        }
    }

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.find_global_variables(local_variables)
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.find_global_variables(local_variables)
            }
        }
    }

    pub fn convert_types(&self, convert: &impl Fn(&Type) -> Type) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.convert_types(convert).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.convert_types(convert).into()
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
