use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordElementOperation {
    type_: Type,
    key: String,
    argument: Rc<Expression>,
    source_information: Rc<SourceInformation>,
}

impl RecordElementOperation {
    pub fn new(
        type_: impl Into<Type>,
        key: impl Into<String>,
        argument: impl Into<Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            key: key.into(),
            argument: Rc::new(argument.into()),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            &self.key,
            self.argument.convert_expressions(convert)?,
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.convert_types(convert)?,
            &self.key,
            self.argument.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
