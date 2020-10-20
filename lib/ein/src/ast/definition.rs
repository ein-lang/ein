use super::expression::Expression;
use super::function_definition::*;
use super::value_definition::*;
use crate::types::Type;

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

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::FunctionDefinition(function_definition) => {
                function_definition.transform_expressions(transform)?.into()
            }
            Self::ValueDefinition(value_definition) => {
                value_definition.transform_expressions(transform)?.into()
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
            Self::ValueDefinition(value_definition) => {
                value_definition.transform_types(transform)?.into()
            }
        })
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
