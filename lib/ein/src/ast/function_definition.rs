use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<String>,
    body: Expression,
    type_: Type,
    source_information: Rc<SourceInformation>,
}

impl FunctionDefinition {
    pub fn new(
        name: impl Into<String>,
        arguments: Vec<String>,
        body: impl Into<Expression>,
        type_: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            name: name.into(),
            arguments,
            body: body.into(),
            type_: type_.into(),
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
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

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.arguments.clone(),
            self.body.convert_expressions(convert)?,
            self.type_.clone(),
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.arguments.clone(),
            self.body.convert_types(convert)?,
            self.type_.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
