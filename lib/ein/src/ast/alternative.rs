use super::definition::Definition;
use super::expression::Expression;
use super::pattern::Pattern;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Alternative {
    pattern: Pattern,
    expression: Expression,
    source_information: Rc<SourceInformation>,
}

impl Alternative {
    pub fn new(
        pattern: impl Into<Pattern>,
        expression: impl Into<Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            pattern: pattern.into(),
            expression: expression.into(),
            source_information: source_information.into(),
        }
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.substitute_type_variables(substitutions),
            self.source_information.clone(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.convert_definitions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.convert_expressions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.convert_types(convert),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.pattern.clone(),
            self.expression.resolve_reference_types(environment),
            self.source_information.clone(),
        )
    }
}
