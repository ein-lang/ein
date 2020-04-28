use super::alternative::Alternative;
use super::expression::Expression;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    argument: Rc<Expression>,
    alternatives: Vec<Alternative>,
}

impl Case {
    pub fn new(argument: impl Into<Expression>, alternatives: Vec<Alternative>) -> Self {
        Self {
            argument: Rc::new(argument.into()),
            alternatives,
        }
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn alternatives(&self) -> &[Alternative] {
        &self.alternatives
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.argument.convert_expressions(convert)?,
            self.alternatives
                .iter()
                .map(|alternative| alternative.convert_expressions(convert))
                .collect::<Result<_, _>>()?,
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.argument.convert_types(convert)?,
            self.alternatives
                .iter()
                .map(|alternative| alternative.convert_types(convert))
                .collect::<Result<_, _>>()?,
        ))
    }
}
