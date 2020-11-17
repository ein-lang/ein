use super::expression::Expression;
use super::function_definition::*;
use super::variable_definition::*;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    FunctionDefinition(FunctionDefinition),
    VariableDefinition(VariableDefinition),
}

impl Definition {
    pub fn name(&self) -> &str {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition.name(),
            Self::VariableDefinition(variable_definition) => variable_definition.name(),
        }
    }

    pub fn type_(&self) -> &Type {
        match self {
            Self::FunctionDefinition(function_definition) => function_definition.type_(),
            Self::VariableDefinition(variable_definition) => variable_definition.type_(),
        }
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.source_information()
            }
            Self::VariableDefinition(variable_definition) => {
                variable_definition.source_information()
            }
        }
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.transform_expressions(transform)?.into()
            }
            Self::VariableDefinition(variable_definition) => {
                variable_definition.transform_expressions(transform)?.into()
            }
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.transform_types(transform)?.into()
            }
            Self::VariableDefinition(variable_definition) => {
                variable_definition.transform_types(transform)?.into()
            }
        })
    }
}

impl From<FunctionDefinition> for Definition {
    fn from(function_definition: FunctionDefinition) -> Self {
        Definition::FunctionDefinition(function_definition)
    }
}

impl From<VariableDefinition> for Definition {
    fn from(variable_definition: VariableDefinition) -> Self {
        Definition::VariableDefinition(variable_definition)
    }
}
