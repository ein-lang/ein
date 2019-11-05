use super::expression::Expression;
use super::function_definition::FunctionDefinition;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct LetFunctions {
    definitions: Vec<FunctionDefinition>,
    expression: Box<Expression>,
}

impl LetFunctions {
    pub fn new(definitions: Vec<FunctionDefinition>, expression: impl Into<Expression>) -> Self {
        Self {
            definitions,
            expression: Box::new(expression.into()),
        }
    }

    pub fn definitions(&self) -> &[FunctionDefinition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        let mut names = names.clone();

        for definition in &self.definitions {
            names.remove(definition.name());
        }

        let mut definitions = Vec::with_capacity(self.definitions.len());

        for definition in &self.definitions {
            definitions.push(definition.rename_variables(&names));
        }

        Self::new(definitions, self.expression.rename_variables(&names))
    }

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        let mut local_variables = local_variables.clone();

        local_variables.extend(
            self.definitions
                .iter()
                .map(|definition| definition.name().into())
                .collect::<HashSet<_>>(),
        );

        self.definitions.iter().fold(
            self.expression.find_global_variables(&local_variables),
            |mut global_variables, argument| {
                global_variables.extend(argument.find_global_variables(&local_variables));
                global_variables
            },
        )
    }
}
