use super::definition::Definition;
use super::expression::Expression;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    condition: Rc<Expression>,
    then: Rc<Expression>,
    else_: Rc<Expression>,
}

impl If {
    pub fn new(
        condition: impl Into<Expression>,
        then: impl Into<Expression>,
        else_: impl Into<Expression>,
    ) -> Self {
        Self {
            condition: Rc::new(condition.into()),
            then: Rc::new(then.into()),
            else_: Rc::new(else_.into()),
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then(&self) -> &Expression {
        &self.then
    }

    pub fn else_(&self) -> &Expression {
        &self.else_
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.condition.substitute_type_variables(substitutions),
            self.then.substitute_type_variables(substitutions),
            self.else_.substitute_type_variables(substitutions),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.condition.convert_definitions(convert),
            self.then.convert_definitions(convert),
            self.else_.convert_definitions(convert),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.condition.convert_expressions(convert),
            self.then.convert_expressions(convert),
            self.else_.convert_expressions(convert),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.condition.convert_types(convert),
            self.then.convert_types(convert),
            self.else_.convert_types(convert),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.condition.resolve_reference_types(environment),
            self.then.resolve_reference_types(environment),
            self.else_.resolve_reference_types(environment),
        )
    }
}
