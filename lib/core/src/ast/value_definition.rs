use super::expression::Expression;
use crate::types::{self, Type};
use std::collections::{HashMap, HashSet};

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

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        Self::new(
            self.name.clone(),
            self.body.rename_variables(names),
            self.type_.clone(),
        )
    }

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        self.body.find_global_variables(&local_variables)
    }

    pub fn convert_types(&self, convert: &impl Fn(&Type) -> Type) -> Self {
        Self::new(
            self.name.clone(),
            self.body.convert_types(convert),
            convert(&self.type_.clone().into()).into_value().unwrap(),
        )
    }
}
