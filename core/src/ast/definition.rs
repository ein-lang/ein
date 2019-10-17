use super::function_definition::*;
use super::value_definition::*;
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

    pub fn rename_variables(&self, names: &HashMap<&str, String>) -> Self {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.rename_variables(names).into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.rename_variables(names).into()
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
