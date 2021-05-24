use super::expression::Expression;
use crate::{
    debug::SourceInformation,
    types::{self, Type},
};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    type_: Type,
    function: Arc<Expression>,
    argument: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl Application {
    pub fn new(
        function: impl Into<Expression>,
        argument: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        let source_information = source_information.into();

        Self::with_type(
            types::Unknown::new(source_information.clone()),
            function,
            argument,
            source_information,
        )
    }

    pub fn with_type(
        type_: impl Into<Type>,
        function: impl Into<Expression>,
        argument: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            function: Arc::new(function.into()),
            argument: Arc::new(argument.into()),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn function(&self) -> &Expression {
        &self.function
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
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
            self.function.transform_expressions(transform)?,
            self.argument.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_.transform_types(transform)?,
            self.function.transform_types(transform)?,
            self.argument.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
