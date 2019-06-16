use super::function_definition::*;
use super::variable_definition::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    VariableDefinition(VariableDefinition),
}

impl From<FunctionDefinition> for Definition {
    fn from(function_definition: FunctionDefinition) -> Self {
        Definition::FunctionDefinition(function_definition)
    }
}

impl From<VariableDefinition> for Definition {
    fn from(function_definition: VariableDefinition) -> Self {
        Definition::VariableDefinition(function_definition)
    }
}
