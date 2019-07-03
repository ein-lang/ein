use super::expression::Expression;
use crate::types;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueDefinition {
    name: String,
    body: Expression,
    type_: types::Value,
}

impl ValueDefinition {
    pub fn new(name: impl Into<String>, body: Expression, type_: types::Value) -> Self {
        Self {
            name: name.into(),
            body,
            type_,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }

    pub fn type_(&self) -> &types::Value {
        &self.type_
    }
}
