use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct List {
    element: Rc<Type>,
    source_information: Rc<SourceInformation>,
}

impl List {
    pub fn new(
        element: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            element: Rc::new(element.into()),
            source_information: source_information.into(),
        }
    }

    pub fn element(&self) -> &Type {
        &self.element
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.element.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
