use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct PipeOperation {
    lhs: Arc<Expression>,
    rhs: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl PipeOperation {
    pub fn new(
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            lhs: lhs.into().into(),
            rhs: rhs.into().into(),
            source_information: source_information.into(),
        }
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.lhs.transform_expressions(transform)?,
            self.rhs.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.lhs.transform_types(transform)?,
            self.rhs.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
