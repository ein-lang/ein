use super::expression::Expression;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueDefinition {
    name: String,
    body: Expression,
    type_: Type,
}

impl ValueDefinition {
    pub fn new(name: String, body: Expression, type_: Type) -> Self {
        Self { name, body, type_ }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.name.clone(),
            self.body.substitute_type_variables(substitutions),
            self.type_.substitute_variables(substitutions),
        )
    }
}
