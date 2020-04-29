use super::alternative::Alternative;
use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    type_: Type,
    argument: Rc<Expression>,
    alternatives: Vec<Alternative>,
}

impl Case {
    pub fn new(
        argument: impl Into<Expression>,
        alternatives: Vec<Alternative>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: types::Unknown::new(source_information).into(),
            argument: Rc::new(argument.into()),
            alternatives,
        }
    }

    pub fn with_type(
        type_: impl Into<Type>,
        argument: impl Into<Expression>,
        alternatives: Vec<Alternative>,
    ) -> Self {
        Self {
            type_: type_.into(),
            argument: Rc::new(argument.into()),
            alternatives,
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
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
        Ok(Self {
            type_: self.type_.clone(),
            argument: self.argument.convert_expressions(convert)?.into(),
            alternatives: self
                .alternatives
                .iter()
                .map(|alternative| alternative.convert_expressions(convert))
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            type_: self.type_.convert_types(convert)?,
            argument: self.argument.convert_types(convert)?.into(),
            alternatives: self
                .alternatives
                .iter()
                .map(|alternative| alternative.convert_types(convert))
                .collect::<Result<_, _>>()?,
        })
    }
}
