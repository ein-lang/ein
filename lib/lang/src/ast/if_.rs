use super::expression::Expression;
use crate::{debug::SourceInformation, types::Type};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    condition: Arc<Expression>,
    then: Arc<Expression>,
    else_: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl If {
    pub fn new(
        condition: impl Into<Expression>,
        then: impl Into<Expression>,
        else_: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            condition: Arc::new(condition.into()),
            then: Arc::new(then.into()),
            else_: Arc::new(else_.into()),
            source_information: source_information.into(),
        }
    }

    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    pub fn then(&self) -> &Expression {
        &self.then
    }

    pub fn else_(&self) -> &Expression {
        &self.else_
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.condition.transform_expressions(transform)?,
            self.then.transform_expressions(transform)?,
            self.else_.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.condition.transform_types(transform)?,
            self.then.transform_types(transform)?,
            self.else_.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
