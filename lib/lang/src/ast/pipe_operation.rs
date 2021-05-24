use super::expression::Expression;
use crate::{
    debug::SourceInformation,
    types::{self, Type},
};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct PipeOperation {
    type_: Type,
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
        let source_information = source_information.into();

        Self::with_type(
            types::Unknown::new(source_information.clone()),
            lhs,
            rhs,
            source_information,
        )
    }

    pub fn with_type(
        type_: impl Into<Type>,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            lhs: lhs.into().into(),
            rhs: rhs.into().into(),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
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
        Ok(Self::with_type(
            self.type_.clone(),
            self.lhs.transform_expressions(transform)?,
            self.rhs.transform_expressions(transform)?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::with_type(
            self.type_.transform_types(transform)?,
            self.lhs.transform_types(transform)?,
            self.rhs.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
