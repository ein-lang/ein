use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordElementOperator {
    key: String,
    source_information: Rc<SourceInformation>,
}

impl RecordElementOperator {
    pub fn new(
        key: impl Into<String>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        let source_information = source_information.into();

        Self {
            key: key.into(),
            source_information,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }
}
