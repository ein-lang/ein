use super::definition::*;
use super::expression::*;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    definitions: Vec<Definition>,
    expression: Rc<Expression>,
}

impl Let {
    pub fn new(definitions: Vec<Definition>, expression: impl Into<Expression>) -> Self {
        Self {
            definitions,
            expression: Rc::new(expression.into()),
        }
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.substitute_type_variables(substitutions))
                .collect::<Vec<_>>(),
            self.expression.substitute_type_variables(substitutions),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_definitions(convert))
                .collect(),
            self.expression.convert_definitions(convert),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.definitions
                .iter()
                .map(|definition| definition.convert_expressions(convert))
                .collect(),
            self.expression.convert_expressions(convert),
        )
    }
}
