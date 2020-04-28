use super::Type;
use crate::debug::SourceInformation;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
#[derivative(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Record {
    // Record ID's are enough to compare them and elements are not normalized
    // possibly.
    id: Option<String>,
    #[derivative(Hash = "ignore")]
    #[derivative(Ord = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    elements: BTreeMap<String, Type>,
    source_information: Rc<SourceInformation>,
}

impl Record {
    pub fn new(
        id: impl Into<String>,
        elements: BTreeMap<String, Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            id: Some(id.into()),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn without_id(
        elements: BTreeMap<String, Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            id: None,
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn elements(&self) -> &BTreeMap<String, Type> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            id: self.id.clone(),
            elements: self
                .elements
                .iter()
                .map(|(name, type_)| Ok((name.into(), type_.convert_types(convert)?)))
                .collect::<Result<_, _>>()?,
            source_information: self.source_information.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

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
