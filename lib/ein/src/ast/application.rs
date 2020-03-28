use super::definition::Definition;
use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    function: Rc<Expression>,
    argument: Rc<Expression>,
    source_information: Rc<SourceInformation>,
}

impl Application {
    pub fn new(
        function: impl Into<Expression>,
        argument: impl Into<Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            function: Rc::new(function.into()),
            argument: Rc::new(argument.into()),
            source_information: source_information.into(),
        }
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_definitions(&self, convert: &mut impl FnMut(&Definition) -> Definition) -> Self {
        Self::new(
            self.function.convert_definitions(convert),
            self.argument.convert_definitions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        Self::new(
            self.function.convert_expressions(convert),
            self.argument.convert_expressions(convert),
            self.source_information.clone(),
        )
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.function.convert_types(convert),
            self.argument.convert_types(convert),
            self.source_information.clone(),
        )
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        Self::new(
            self.function.resolve_reference_types(environment),
            self.argument.resolve_reference_types(environment),
            self.source_information.clone(),
        )
    }
}
