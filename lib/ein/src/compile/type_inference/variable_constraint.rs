use crate::debug::SourceInformation;
use crate::types::{self, Type};
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

    pub fn to_type(&self) -> Type {
        if self.lower_types.iter().any(|type_| type_.is_any()) {
            types::Any::new(self.source_information.clone()).into()
        } else if !self.lower_types.is_empty() {
            types::Union::new(
                self.lower_types.iter().cloned().collect(),
                self.source_information.clone(),
            )
            .into()
        } else if !self.upper_types.is_empty() {
            // TODO Calculate the minimal type from upper types?
            self.upper_types.iter().next().unwrap().clone()
        } else {
            types::Unknown::new(self.source_information.clone()).into()
        }
    }
}
