use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::BTreeSet;
use std::sync::Arc;

pub struct VariableConstraint {
    lower_types: BTreeSet<Type>,
    upper_types: BTreeSet<Type>,
    source_information: Arc<SourceInformation>,
}

impl VariableConstraint {
    pub fn new(source_information: Arc<SourceInformation>) -> Self {
        Self {
            lower_types: Default::default(),
            upper_types: Default::default(),
            source_information,
        }
    }

    pub fn lower_types(&self) -> &BTreeSet<Type> {
        &self.lower_types
    }

    pub fn upper_types(&self) -> &BTreeSet<Type> {
        &self.upper_types
    }

    pub fn add_lower_type(&mut self, type_: &Type) {
        self.lower_types.insert(type_.clone());
    }

    pub fn add_upper_type(&mut self, type_: &Type) {
        self.upper_types.insert(type_.clone());
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }
}
