use super::Type;
use crate::debug::SourceInformation;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
#[derivative(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Record {
    id: String,
    // Record ID's are enough to compare them and elements are not normalized
    // possibly.
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
            id: id.into(),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn elements(&self) -> &BTreeMap<String, Type> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            &self.id,
            self.elements
                .iter()
                .map(|(name, type_)| (name.into(), type_.convert_types(convert)))
                .collect(),
            self.source_information.clone(),
        )
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
