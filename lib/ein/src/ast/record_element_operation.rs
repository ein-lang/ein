use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordElementOperation {
    key: String,
    argument: Box<Expression>,
    type_: Type,
    source_information: Rc<SourceInformation>,
}

impl RecordElementOperation {
    pub fn new(
        key: impl Into<String>,
        argument: impl Into<Expression>,
        type_: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        let source_information = source_information.into();

        RecordElementOperation {
            key: key.into(),
            argument: Box::new(argument.into()),
            type_: type_.into(),
            source_information,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self {
            key: self.key.clone(),
            argument: self.argument.convert_expressions(convert).into(),
            type_: self.type_.clone(),
            source_information: self.source_information.clone(),
        }
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self {
            key: self.key.clone(),
            argument: self.argument.convert_types(convert).into(),
            type_: convert(&self.type_),
            source_information: self.source_information.clone(),
        }
    }
}
