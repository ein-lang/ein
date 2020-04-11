use super::definition::*;
use super::expression::*;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    definitions: Vec<Definition>,
    expression: Rc<Expression>,
}

impl Let {
    pub fn new(definitions: Vec<Definition>, expression: impl Into<Expression>) -> Self {
        Self {
            definitions,
            expression: Rc::new(expression.into()),
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
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

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_types(convert))
                .collect(),
            self.expression.convert_types(convert),
        )
    }
}
