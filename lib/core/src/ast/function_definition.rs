use super::expression::Expression;
use super::Argument;
use crate::types;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    environment: Vec<Argument>,
    arguments: Vec<Argument>,
    body: Expression,
    result_type: types::Value,
    type_: types::Function,
}

impl FunctionDefinition {
    pub fn new(
        name: impl Into<String>,
        environment: Vec<Argument>,
        arguments: Vec<Argument>,
        body: impl Into<Expression>,
        result_type: types::Value,
    ) -> Self {
        Self {
            type_: types::Function::new(
                arguments
                    .iter()
                    .map(|argument| argument.type_().clone())
                    .collect(),
                result_type.clone(),
            ),
            name: name.into(),
            environment,
            arguments,
            body: body.into(),
            result_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn environment(&self) -> &[Argument] {
        &self.environment
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }

    pub fn result_type(&self) -> &types::Value {
        &self.result_type
    }

    pub fn type_(&self) -> &types::Function {
        &self.type_
    }

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        let mut names = names.clone();

        names.remove(self.name.as_str());

        for free_variable in &self.environment {
            names.remove(free_variable.name());
        }

        for argument in &self.arguments {
            names.remove(argument.name());
        }

        Self::new(
            self.name.clone(),
            self.environment.clone(),
            self.arguments.clone(),
            self.body.rename_variables(&names),
            self.result_type.clone(),
        )
    }

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        let mut local_variables = local_variables.clone();

        local_variables.insert(self.name.clone());

        local_variables.extend(
            self.environment
                .iter()
                .map(|argument| argument.name().into())
                .collect::<HashSet<_>>(),
        );

        local_variables.extend(
            self.arguments
                .iter()
                .map(|argument| argument.name().into())
                .collect::<HashSet<_>>(),
        );

        self.body.find_global_variables(&local_variables)
    }
}
