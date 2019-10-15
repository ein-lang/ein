use super::expression::Expression;
use crate::types;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueDefinition {
    name: String,
    body: Expression,
    type_: types::Value,
}

impl ValueDefinition {
    pub fn new(name: impl Into<String>, body: impl Into<Expression>, type_: types::Value) -> Self {
        Self {
            name: name.into(),
            body: body.into(),
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

    pub fn rename_variables(&self, names: &HashMap<&str, String>) -> Self {
        Self::new(
            self.name.clone(),
            self.body.rename_variables(names),
            self.type_.clone(),
        )
    }
}
