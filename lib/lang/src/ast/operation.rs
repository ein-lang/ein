use super::expression::Expression;
use super::generic_operation::GenericOperation;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Generic(GenericOperation),
}

impl Operation {
    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::Generic(operation) => operation.source_information(),
        }
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Generic(operation) => operation.transform_expressions(transform)?.into(),
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Generic(operation) => operation.transform_types(transform)?.into(),
        })
    }
}

impl From<GenericOperation> for Operation {
    fn from(operation: GenericOperation) -> Self {
        Self::Generic(operation)
    }
}
