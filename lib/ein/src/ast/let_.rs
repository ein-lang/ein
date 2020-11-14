use super::definition::*;
use super::expression::*;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    definitions: Vec<Definition>,
    expression: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl Let {
    pub fn new(
        definitions: Vec<Definition>,
        expression: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            definitions,
            expression: Arc::new(expression.into()),
            source_information: source_information.into(),
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn has_functions(&self) -> bool {
        self.definitions.iter().any(|definition| match definition {
            Definition::FunctionDefinition(_) => true,
            Definition::VariableDefinition(variable_definition) => {
                variable_definition.type_().is_function()
            }
        })
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.transform_expressions(transform))
                .collect::<Result<_, _>>()?,
            self.expression.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.expression.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
