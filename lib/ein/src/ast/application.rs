use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
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
        Self {
            function: Arc::new(function.into()),
            argument: Arc::new(argument.into()),
            source_information: source_information.into(),
        }
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

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.function.convert_expressions(convert)?,
            self.argument.convert_expressions(convert)?,
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.function.convert_types(convert)?,
            self.argument.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
