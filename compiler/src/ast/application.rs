use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    function: Box<Expression>,
    arguments: Vec<Expression>,
}

impl Application {
    pub fn new(function: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            function: Box::new(function),
            arguments,
        }
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn arguments(&self) -> &[Expression] {
        &self.arguments
    }
}
