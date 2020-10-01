use super::super::error::CompileError;
use super::super::union_type_simplifier::UnionTypeSimplifier;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub struct VariableSubstitutor {
    union_type_simplifier: Arc<UnionTypeSimplifier>,
    substitutions: HashMap<usize, Type>,
}

impl VariableSubstitutor {
    pub fn new(
        union_type_simplifier: Arc<UnionTypeSimplifier>,
        substitutions: HashMap<usize, Type>,
    ) -> Arc<Self> {
        Self {
            union_type_simplifier,
            substitutions,
        }
        .into()
    }

    pub fn substitute(&self, type_: &Type) -> Result<Type, CompileError> {
        if let Type::Variable(variable) = type_ {
            self.substitutions.get(&variable.id()).ok_or_else(|| {
                CompileError::TypeNotInferred(variable.source_information().clone())
            })?
        } else {
            type_
        }
        .convert_types(&mut |type_| self.union_type_simplifier.simplify(type_))
    }
}
