use super::expression::Expression;
use super::list_element::ListElement;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    type_: Type,
    elements: Vec<ListElement>,
    source_information: Arc<SourceInformation>,
}

impl List {
    pub fn new(
        elements: Vec<ListElement>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        let source_information: Arc<_> = source_information.into();

        Self {
            type_: types::Unknown::new(source_information.clone()).into(),
            elements,
            source_information,
        }
    }

    pub fn with_type(
        type_: impl Into<Type>,
        elements: Vec<ListElement>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn elements(&self) -> &[ListElement] {
        &self.elements
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_().clone(),
            self.elements()
                .iter()
                .map(|element| element.convert_expressions(convert))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_.convert_types(convert)?,
            self.elements()
                .iter()
                .map(|element| element.convert_types(convert))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
