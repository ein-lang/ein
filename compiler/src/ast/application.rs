use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    function: Box<Expression>,
    argument: Box<Expression>,
}

impl Application {
    pub fn new(function: Expression, argument: Expression) -> Self {
        Self {
            function: Box::new(function),
            argument: Box::new(argument),
        }
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }
}
