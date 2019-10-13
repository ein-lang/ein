use super::expression::Expression;
use super::variable::Variable;

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
}
