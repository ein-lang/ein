use super::{expression::Expression, variable_definition::VariableDefinition};
use crate::{
    debug::SourceInformation,
    types::{self, Type},
};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct LetError {
    type_: Type,
    definitions: Vec<VariableDefinition>,
    expression: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl LetError {
    pub fn new(
        definitions: Vec<VariableDefinition>,
        expression: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        let source_information = source_information.into();

        Self::with_type(
            types::Unknown::new(source_information.clone()),
            definitions,
            expression,
            source_information,
        )
    }

    pub fn with_type(
        type_: impl Into<Type>,
        definitions: Vec<VariableDefinition>,
        expression: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            definitions,
            expression: Arc::new(expression.into()),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn definitions(&self) -> &[VariableDefinition] {
        &self.definitions
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.transform_expressions(transform))
                .collect::<Result<_, _>>()?,
            self.expression.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            transform(&self.type_)?,
            self.definitions
                .iter()
                .map(|definition| definition.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.expression.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
