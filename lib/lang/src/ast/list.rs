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

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_().clone(),
            self.elements()
                .iter()
                .map(|element| element.transform_expressions(transform))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_.transform_types(transform)?,
            self.elements()
                .iter()
                .map(|element| element.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
