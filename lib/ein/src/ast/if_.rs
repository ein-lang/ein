use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    condition: Rc<Expression>,
    then: Rc<Expression>,
    else_: Rc<Expression>,
    source_information: Rc<SourceInformation>,
}

impl If {
    pub fn new(
        condition: impl Into<Expression>,
        then: impl Into<Expression>,
        else_: impl Into<Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            condition: Rc::new(condition.into()),
            then: Rc::new(then.into()),
            else_: Rc::new(else_.into()),
            source_information: source_information.into(),
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

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.condition.convert_expressions(convert),
            self.then.convert_expressions(convert),
            self.else_.convert_expressions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.condition.convert_types(convert),
            self.then.convert_types(convert),
            self.else_.convert_types(convert),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.condition.resolve_reference_types(environment),
            self.then.resolve_reference_types(environment),
            self.else_.resolve_reference_types(environment),
            self.source_information.clone(),
        )
    }
}
