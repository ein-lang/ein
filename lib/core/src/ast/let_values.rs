use super::expression::Expression;
use super::value_definition::ValueDefinition;
use crate::types::Type;
use std::collections::{HashMap, HashSet};

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

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        let mut local_variables = local_variables.clone();
        let mut global_variables = HashSet::new();

        for definition in &self.definitions {
            global_variables.extend(definition.find_global_variables(&local_variables));
            local_variables.insert(definition.name().into());
        }

        global_variables.extend(self.expression.find_global_variables(&local_variables));

        global_variables
    }

    pub fn convert_types(&self, convert: &impl Fn(&Type) -> Type) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_types(convert))
                .collect(),
            self.expression.convert_types(convert),
        )
    }
}
