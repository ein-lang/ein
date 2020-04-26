use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Union {
    types: BTreeSet<Type>,
    source_information: Rc<SourceInformation>,
}

impl Union {
    pub fn new(types: Vec<Type>, source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            types: types.into_iter().collect(),
            source_information: source_information.into(),
        }
    }

    pub fn types(&self) -> &BTreeSet<Type> {
        &self.types
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.types
                .iter()
                .map(|type_| type_.convert_types(convert))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
