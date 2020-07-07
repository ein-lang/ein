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

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect::<Result<_, _>>()?,
            self.expression.convert_expressions(convert)?,
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_types(convert))
                .collect::<Result<_, _>>()?,
            self.expression.convert_types(convert)?,
        ))
    }
}
