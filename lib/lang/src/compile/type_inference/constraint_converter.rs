use super::{
    super::{error::CompileError, reference_type_resolver::ReferenceTypeResolver},
    variable_constraint::VariableConstraint,
};
use crate::types::{self, Type};
use std::sync::Arc;

pub struct ConstraintConverter {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl ConstraintConverter {
    pub fn new(reference_type_resolver: Arc<ReferenceTypeResolver>) -> Arc<Self> {
        Self {
            reference_type_resolver,
        }
        .into()
    }

    pub fn convert(&self, constraint: &VariableConstraint) -> Result<Type, CompileError> {
        Ok(
            if constraint
                .lower_types()
                .iter()
                .map(|type_| self.reference_type_resolver.is_any(type_))
                .collect::<Result<Vec<bool>, _>>()?
                .into_iter()
                .any(|ok| ok)
            {
                types::Any::new(constraint.source_information().clone()).into()
            } else if !constraint.lower_types().is_empty() {
                types::Union::new(
                    constraint.lower_types().iter().cloned().collect(),
                    constraint.source_information().clone(),
                )
                .into()
            } else if !constraint.upper_types().is_empty() {
                // TODO Calculate intersection types from upper types?
                constraint.upper_types().iter().next().unwrap().clone()
            } else {
                types::Unknown::new(constraint.source_information().clone()).into()
            },
        )
    }
}
