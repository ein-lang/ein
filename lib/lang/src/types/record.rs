use super::Type;
use crate::debug::SourceInformation;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
#[derivative(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Record {
    name: String,
    #[derivative(
        Hash = "ignore",
        Ord = "ignore",
        PartialEq = "ignore",
        PartialOrd = "ignore"
    )]
    elements: BTreeMap<String, Type>,
    source_information: Arc<SourceInformation>,
}

impl Record {
    pub fn new(
        name: impl Into<String>,
        elements: BTreeMap<String, Type>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn elements(&self) -> &BTreeMap<String, Type> {
        &self.elements
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            name: self.name.clone(),
            elements: self
                .elements
                .iter()
                .map(|(name, type_)| Ok((name.into(), type_.transform_types(transform)?)))
                .collect::<Result<_, _>>()?,
            source_information: self.source_information.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{super::*, *};

    #[test]
    fn equal() {
        assert_eq!(
            Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    Reference::new("Foo", SourceInformation::dummy()).into()
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy()
            ),
            Record::new(
                "Foo",
                vec![(
                    "foo".into(),
                    Record::new(
                        "Foo",
                        vec![(
                            "foo".into(),
                            Reference::new("Foo", SourceInformation::dummy()).into()
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy()
                    )
                    .into(),
                )]
                .into_iter()
                .collect(),
                SourceInformation::dummy()
            ),
        );
    }
}
