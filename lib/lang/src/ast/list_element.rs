use super::expression::Expression;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum ListElement {
    Multiple(Expression),
    Single(Expression),
}

impl ListElement {
    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Multiple(expression) => {
                Self::Multiple(expression.transform_expressions(transform)?)
            }
            Self::Single(expression) => Self::Single(expression.transform_expressions(transform)?),
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Multiple(expression) => Self::Multiple(expression.transform_types(transform)?),
            Self::Single(expression) => Self::Single(expression.transform_types(transform)?),
        })
    }
}
