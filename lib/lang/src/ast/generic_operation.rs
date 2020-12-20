use super::expression::Expression;
use super::operator::Operator;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct GenericOperation {
    type_: Type,
    operator: Operator,
    lhs: Arc<Expression>,
    rhs: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl GenericOperation {
    pub fn new(
        operator: Operator,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        let source_information: Arc<_> = source_information.into();

        Self::with_type(
            types::Unknown::new(source_information.clone()),
            operator,
            lhs,
            rhs,
            source_information,
        )
    }

    pub fn with_type(
        type_: impl Into<Type>,
        operator: Operator,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            operator,
            lhs: Arc::new(lhs.into()),
            rhs: Arc::new(rhs.into()),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn operator(&self) -> Operator {
        self.operator
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
            self.operator,
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
            self.operator,
            self.lhs.transform_types(transform)?,
            self.rhs.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
