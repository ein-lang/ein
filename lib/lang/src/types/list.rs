use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct List {
    element: Arc<Type>,
    source_information: Arc<SourceInformation>,
}

impl List {
    pub fn new(
        element: impl Into<Type>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            element: Arc::new(element.into()),
            source_information: source_information.into(),
        }
    }

    pub fn element(&self) -> &Type {
        &self.element
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.element.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
