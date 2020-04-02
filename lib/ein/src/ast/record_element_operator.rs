use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordElementOperator {
    key: String,
    type_: Type,
    source_information: Rc<SourceInformation>,
}

impl RecordElementOperator {
    pub fn new(
        key: impl Into<String>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        let source_information = source_information.into();

        Self {
            key: key.into(),
            type_: types::Unknown::new(source_information.clone()).into(),
            source_information,
        }
    }

    #[cfg(test)]
    pub fn with_type(
        key: impl Into<String>,
        type_: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            key: key.into(),
            type_: type_.into(),
            source_information: source_information.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self {
            key: self.key.clone(),
            type_: convert(&self.type_),
            source_information: self.source_information.clone(),
        }
    }
}
