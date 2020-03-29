use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordUpdate {
    original: Box<Expression>,
    elements: BTreeMap<String, Expression>,
    source_information: Rc<SourceInformation>,
}

impl RecordUpdate {
    pub fn new(
        original: impl Into<Expression>,
        elements: BTreeMap<String, Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            original: Box::new(original.into()),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn original(&self) -> &Expression {
        &self.original
    }

    pub fn elements(&self) -> &BTreeMap<String, Expression> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.original.convert_expressions(convert),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_expressions(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.original.convert_types(convert),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_types(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }
}
