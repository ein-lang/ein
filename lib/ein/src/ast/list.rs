use super::expression::Expression;
use super::list_element::ListElement;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    elements: Vec<ListElement>,
    source_information: Rc<SourceInformation>,
}

impl List {
    pub fn new(
        elements: Vec<ListElement>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn elements(&self) -> &[ListElement] {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
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
        Ok(Self::new(
            self.elements()
                .iter()
                .map(|element| element.convert_types(convert))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
