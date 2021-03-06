use super::expression::Expression;
use crate::{debug::SourceInformation, types::Type};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<String>,
    body: Expression,
    type_: Type,
    source_information: Arc<SourceInformation>,
}

impl FunctionDefinition {
    pub fn new(
        name: impl Into<String>,
        arguments: Vec<String>,
        body: impl Into<Expression>,
        type_: impl Into<Type>,
        source_information: impl Into<Arc<SourceInformation>>,
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

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.arguments.clone(),
            self.body.transform_expressions(transform)?,
            self.type_.clone(),
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.arguments.clone(),
            self.body.transform_types(transform)?,
            self.type_.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
