use super::expression::Expression;
use super::function_definition::FunctionDefinition;

#[derive(Clone, Debug, PartialEq)]
pub struct LetFunctions {
    definitions: Vec<FunctionDefinition>,
    expression: Box<Expression>,
}

impl LetFunctions {
    pub fn new(definitions: Vec<FunctionDefinition>, expression: Expression) -> Self {
        Self {
            definitions,
            expression: Box::new(expression),
        }
    }

    pub fn definitions(&self) -> &[FunctionDefinition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}
