use super::definition::Definition;
use super::expression::Expression;
use super::operator::Operator;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    operator: Operator,
    lhs: Rc<Expression>,
    rhs: Rc<Expression>,
    source_information: Rc<SourceInformation>,
}

impl Operation {
    pub fn new(
        operator: Operator,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            operator,
            lhs: Rc::new(lhs.into()),
            rhs: Rc::new(rhs.into()),
            source_information: source_information.into(),
        }
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.operator,
            self.lhs.convert_definitions(convert),
            self.rhs.convert_definitions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.operator,
            self.lhs.convert_expressions(convert),
            self.rhs.convert_expressions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.operator,
            self.lhs.convert_types(convert),
            self.rhs.convert_types(convert),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.operator,
            self.lhs.resolve_reference_types(environment),
            self.rhs.resolve_reference_types(environment),
            self.source_information.clone(),
        )
    }
}
