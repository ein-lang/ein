use super::expression::Expression;
use super::value_definition::ValueDefinition;

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
}
