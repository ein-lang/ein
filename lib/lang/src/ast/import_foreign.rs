use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct ImportForeign {
    name: String,
    foreign_name: String,
    type_: Type,
    source_information: Arc<SourceInformation>,
}

impl ImportForeign {
    pub fn new(
        name: impl Into<String>,
        foreign_name: impl Into<String>,
        type_: impl Into<Type>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            foreign_name: foreign_name.into(),
            type_: type_.into(),
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn foreign_name(&self) -> &str {
        &self.foreign_name
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.foreign_name.clone(),
            self.type_.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
