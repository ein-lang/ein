use super::expression::Expression;
use super::variable::Variable;
use crate::types::Type;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    function: Variable,
    arguments: Vec<Expression>,
}

impl Application {
    pub fn new(function: Variable, arguments: Vec<Expression>) -> Self {
        Self {
            function,
            arguments,
        }
    }

    pub fn function(&self) -> &Variable {
        &self.function
    }

    pub fn arguments(&self) -> &[Expression] {
        &self.arguments
    }

    pub fn rename_variables(&self, names: &HashMap<String, String>) -> Self {
        Self::new(
            self.function.rename_variables(names),
            self.arguments
                .iter()
                .map(|argument| argument.rename_variables(names))
                .collect(),
        )
    }

    pub fn find_global_variables(&self, local_variables: &HashSet<String>) -> HashSet<String> {
        self.arguments.iter().fold(
            self.function.find_global_variables(local_variables),
            |mut global_variables, argument| {
                global_variables.extend(argument.find_global_variables(local_variables));
                global_variables
            },
        )
    }

    pub fn convert_types(&self, convert: &impl Fn(&Type) -> Type) -> Self {
        Self {
            function: self.function.clone(),
            arguments: self
                .arguments
                .iter()
                .map(|argument| argument.convert_types(convert))
                .collect(),
        }
    }
}
