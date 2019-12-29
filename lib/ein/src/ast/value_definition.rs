use super::definition::Definition;
use super::expression::Expression;
use crate::debug::*;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueDefinition {
    name: String,
    body: Expression,
    type_: Type,
    source_information: Rc<SourceInformation>,
}

impl ValueDefinition {
    pub fn new(
        name: impl Into<String>,
        body: impl Into<Expression>,
        type_: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            body: body.into(),
            type_: type_.into(),
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn substitute_type_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.name.clone(),
            self.body.substitute_type_variables(substitutions),
            self.type_.substitute_variables(substitutions),
            self.source_information.clone(),
        )
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.name.clone(),
            self.body.convert_definitions(convert),
            self.type_.clone(),
            self.source_information.clone(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.name.clone(),
            self.body.convert_expressions(convert),
            self.type_.clone(),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.name.clone(),
            self.body.resolve_reference_types(environment),
            self.type_.resolve_reference_types(environment),
            self.source_information.clone(),
        )
    }
}
