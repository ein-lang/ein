use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::collections::BTreeSet;
use std::rc::Rc;

pub struct VariableConstraint {
    lower_types: BTreeSet<Type>,
    upper_types: BTreeSet<Type>,
    source_information: Rc<SourceInformation>,
}

impl VariableConstraint {
    pub fn new(source_information: Rc<SourceInformation>) -> Self {
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
        } else {
            types::Union::new(
                self.lower_types.iter().cloned().collect(),
                self.source_information.clone(),
            )
            .into()
        }
    }
}
