use super::definition::*;
use super::expression::*;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    definitions: Vec<Definition>,
    expression: Arc<Expression>,
}

impl Let {
    pub fn new(definitions: Vec<Definition>, expression: impl Into<Expression>) -> Self {
        Self {
            definitions,
            expression: Arc::new(expression.into()),
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn has_functions(&self) -> bool {
        self.definitions.iter().any(|definition| match definition {
            Definition::FunctionDefinition(_) => true,
            Definition::ValueDefinition(value_definition) => value_definition.type_().is_function(),
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
        ))
    }
}
