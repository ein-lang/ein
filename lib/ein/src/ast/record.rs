use super::definition::Definition;
use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    type_: Type, // Must be record type
    elements: HashMap<String, Expression>,
    source_information: Rc<SourceInformation>,
}

impl Record {
    pub fn new(
        type_: impl Into<Type>,
        elements: HashMap<String, Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn elements(&self) -> &HashMap<String, Expression> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.type_.clone(),
            self.elements
                .iter()
                .map(|(name, expression)| {
                    (
                        name.into(),
                        expression.substitute_type_variables(substitutions),
                    )
                })
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.type_.clone(),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_definitions(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.type_.clone(),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_expressions(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            convert(&self.type_),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_types(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.type_.resolve_reference_types(environment),
            self.elements
                .iter()
                .map(|(name, expression)| {
                    (name.into(), expression.resolve_reference_types(environment))
                })
                .collect(),
            self.source_information.clone(),
        )
    }
}
