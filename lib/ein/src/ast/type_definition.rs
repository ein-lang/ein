use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDefinition {
    name: String,
    type_: Type,
}

impl TypeDefinition {
    pub fn new(name: impl Into<String>, type_: impl Into<Type>) -> Self {
        Self {
            name: name.into(),
            type_: type_.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(&self.name, self.type_.resolve_reference_types(environment))
    }
}
