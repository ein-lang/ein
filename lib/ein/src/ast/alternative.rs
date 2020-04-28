use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    type_: Type,
    name: String,
    expression: Expression,
}

impl Alternative {
    pub fn new(
        type_: impl Into<Type>,
        name: impl Into<String>,
        expression: impl Into<Expression>,
    ) -> Self {
        Self {
            type_: type_.into(),
            name: name.into(),
            expression: expression.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            self.name.clone(),
            self.expression.convert_expressions(convert)?,
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.convert_types(convert)?,
            self.name.clone(),
            self.expression.convert_types(convert)?,
        ))
    }
}
