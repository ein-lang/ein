use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum ListElement {
    Multiple(Expression),
    Single(Expression),
}

impl ListElement {
    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Multiple(expression) => Self::Multiple(expression.convert_expressions(convert)?),
            Self::Single(expression) => Self::Single(expression.convert_expressions(convert)?),
        })
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Multiple(expression) => Self::Multiple(expression.convert_types(convert)?),
            Self::Single(expression) => Self::Single(expression.convert_types(convert)?),
        })
    }
}
