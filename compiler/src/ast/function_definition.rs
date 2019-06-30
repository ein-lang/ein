use super::expression::Expression;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<String>,
    body: Expression,
    type_: Type,
}

impl FunctionDefinition {
    pub fn new<T: Into<Type>>(
        name: String,
        arguments: Vec<String>,
        body: Expression,
        type_: T,
    ) -> Self {
        Self {
            name,
            arguments,
            body,
            type_: type_.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
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
            self.arguments.clone(),
            self.body.substitute_type_variables(substitutions),
            self.type_.substitute_variables(substitutions),
        )
    }
}
