use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    type_: Type,
    expression: Expression,
}

impl Alternative {
    pub fn new(type_: impl Into<Type>, expression: impl Into<Expression>) -> Self {
        Self {
            type_: type_.into(),
            expression: expression.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            self.expression.transform_expressions(transform)?,
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.transform_types(transform)?,
            self.expression.transform_types(transform)?,
        ))
    }
}
