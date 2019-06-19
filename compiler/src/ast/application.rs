use super::expression::Expression;
use crate::types::Type;
use std::collections::HashMap;
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

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.function.substitute_type_variables(substitutions),
            self.argument.substitute_type_variables(substitutions),
        )
    }
}
