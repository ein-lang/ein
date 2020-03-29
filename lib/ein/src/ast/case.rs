use super::alternative::Alternative;
use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    argument: Rc<Expression>,
    alternatives: Vec<Alternative>,
    source_information: Rc<SourceInformation>,
}

impl Case {
    pub fn new(
        argument: impl Into<Expression>,
        alternatives: Vec<Alternative>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            argument: Rc::new(argument.into()),
            alternatives,
            source_information: source_information.into(),
        }
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn alternatives(&self) -> &[Alternative] {
        &self.alternatives
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.argument.convert_expressions(convert),
            self.alternatives
                .iter()
                .map(|alternative| alternative.convert_expressions(convert))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.argument.convert_types(convert),
            self.alternatives
                .iter()
                .map(|alternative| alternative.convert_types(convert))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.argument.resolve_reference_types(environment),
            self.alternatives
                .iter()
                .map(|alternative| alternative.resolve_reference_types(environment))
                .collect(),
            self.source_information.clone(),
        )
    }
}
