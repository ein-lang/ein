use super::super::error::CompileError;
use super::super::type_canonicalizer::TypeCanonicalizer;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub struct VariableSubstitutor {
    type_canonicalizer: Arc<TypeCanonicalizer>,
    substitutions: HashMap<usize, Type>,
}

impl VariableSubstitutor {
    pub fn new(
        type_canonicalizer: Arc<TypeCanonicalizer>,
        substitutions: HashMap<usize, Type>,
    ) -> Arc<Self> {
        Self {
            type_canonicalizer,
            substitutions,
        }
        .into()
    }

    pub fn substitute(&self, type_: &Type) -> Result<Type, CompileError> {
        self.type_canonicalizer
            .canonicalize(if let Type::Variable(variable) = type_ {
                self.substitutions.get(&variable.id()).ok_or_else(|| {
                    CompileError::TypeNotInferred(variable.source_information().clone())
                })?
            } else {
                type_
            })
    }
}
