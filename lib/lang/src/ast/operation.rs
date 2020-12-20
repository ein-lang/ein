use super::arithmetic_operation::ArithmeticOperation;
use super::boolean_operation::BooleanOperation;
use super::expression::Expression;
use super::generic_operation::GenericOperation;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Arithmetic(ArithmeticOperation),
    Boolean(BooleanOperation),
    Generic(GenericOperation),
}

impl Operation {
    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::Arithmetic(operation) => operation.source_information(),
            Self::Boolean(operation) => operation.source_information(),
            Self::Generic(operation) => operation.source_information(),
        }
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Arithmetic(operation) => operation.transform_expressions(transform)?.into(),
            Self::Boolean(operation) => operation.transform_expressions(transform)?.into(),
            Self::Generic(operation) => operation.transform_expressions(transform)?.into(),
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Arithmetic(operation) => operation.transform_types(transform)?.into(),
            Self::Boolean(operation) => operation.transform_types(transform)?.into(),
            Self::Generic(operation) => operation.transform_types(transform)?.into(),
        })
    }
}

impl From<ArithmeticOperation> for Operation {
    fn from(operation: ArithmeticOperation) -> Self {
        Self::Arithmetic(operation)
    }
}

impl From<BooleanOperation> for Operation {
    fn from(operation: BooleanOperation) -> Self {
        Self::Boolean(operation)
    }
}

impl From<GenericOperation> for Operation {
    fn from(operation: GenericOperation) -> Self {
        Self::Generic(operation)
    }
}
