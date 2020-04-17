use crate::types::{self, Type};
use std::collections::BTreeSet;

pub struct VariableConstraint {
    variable: types::Variable,
    lower_types: BTreeSet<Type>,
    upper_types: BTreeSet<Type>,
}

impl VariableConstraint {
    pub fn new(variable: &types::Variable) -> Self {
        Self {
            variable: variable.clone(),
            lower_types: Default::default(),
            upper_types: Default::default(),
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
        types::Union::new(
            self.lower_types.iter().cloned().collect(),
            self.variable.source_information().clone(),
        )
        .into()
    }
}
