use super::definition::*;
use super::expression::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    definitions: Vec<Definition>,
    expression: Box<Expression>,
}

impl Let {
    pub fn new(definitions: Vec<Definition>, expression: Expression) -> Self {
        Self {
            definitions,
            expression: Box::new(expression),
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}
