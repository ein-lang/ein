use super::expression::Expression;
use super::value_definition::ValueDefinition;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct LetValues {
    definitions: Vec<ValueDefinition>,
    expression: Box<Expression>,
}

impl LetValues {
    pub fn new(definitions: Vec<ValueDefinition>, expression: impl Into<Expression>) -> Self {
        Self {
            definitions,
            expression: Box::new(expression.into()),
        }
    }

    pub fn definitions(&self) -> &[ValueDefinition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        let mut names = names.clone();
        let mut definitions = Vec::with_capacity(self.definitions.len());

        for definition in &self.definitions {
            definitions.push(definition.rename_variables(&names));
            names.remove(definition.name());
        }

        Self::new(definitions, self.expression.rename_variables(&names))
    }
}
