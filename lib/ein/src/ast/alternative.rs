use super::expression::Expression;
use super::pattern::Pattern;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    pattern: Pattern,
    expression: Expression,
}

impl Alternative {
    pub fn new(pattern: impl Into<Pattern>, expression: impl Into<Expression>) -> Self {
        Self {
            pattern: pattern.into(),
            expression: expression.into(),
        }
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.convert_expressions(convert),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(self.pattern.clone(), self.expression.convert_types(convert))
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.resolve_reference_types(environment),
        )
    }
}
