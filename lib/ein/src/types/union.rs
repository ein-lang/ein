use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::Arc;

// Do not construct union types during compilation.
// They are allowed to be constructed only on parsing and at the end of type inference to keep them canonical.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Union {
    types: BTreeSet<Type>,
    source_information: Arc<SourceInformation>,
}

impl Union {
    pub fn new(types: Vec<Type>, source_information: impl Into<Arc<SourceInformation>>) -> Self {
        Self {
            types: types.into_iter().collect(),
            source_information: source_information.into(),
        }
    }

    pub fn types(&self) -> &BTreeSet<Type> {
        &self.types
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.types
                .iter()
                .map(|type_| type_.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
