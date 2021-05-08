use super::{record_element::RecordElement, Type};
use crate::debug::SourceInformation;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    elements: Vec<RecordElement>,
    source_information: Arc<SourceInformation>,
}

impl Record {
    pub fn new(
        name: impl Into<String>,
        elements: Vec<RecordElement>,
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

    pub fn elements(&self) -> &[RecordElement] {
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
                .map(|element| {
                    Ok(RecordElement::new(
                        element.name(),
                        element.type_().transform_types(transform)?,
                    ))
                })
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
                vec![RecordElement::new(
                    "foo",
                    Reference::new("Foo", SourceInformation::dummy())
                )],
                SourceInformation::dummy()
            ),
            Record::new(
                "Foo",
                vec![RecordElement::new(
                    "foo",
                    Record::new(
                        "Foo",
                        vec![RecordElement::new(
                            "foo",
                            Reference::new("Foo", SourceInformation::dummy())
                        )],
                        SourceInformation::dummy()
                    ),
                )],
                SourceInformation::dummy()
            ),
        );
    }
}
