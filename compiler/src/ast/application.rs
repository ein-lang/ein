use super::expression::Expression;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    function: Rc<Expression>,
    argument: Rc<Expression>,
}

impl Application {
    pub fn new(function: Expression, argument: Expression) -> Self {
        Self {
            function: Rc::new(function),
            argument: Rc::new(argument),
        }
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }
}
