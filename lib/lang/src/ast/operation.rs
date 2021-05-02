use super::{
    arithmetic_operation::ArithmeticOperation, boolean_operation::BooleanOperation,
    equality_operation::EqualityOperation, expression::Expression, order_operation::OrderOperation,
    pipe_operation::PipeOperation,
};
use crate::{debug::SourceInformation, types::Type};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Arithmetic(ArithmeticOperation),
    Boolean(BooleanOperation),
    Equality(EqualityOperation),
    Order(OrderOperation),
    Pipe(PipeOperation),
}

impl Operation {
    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::Arithmetic(operation) => operation.source_information(),
            Self::Boolean(operation) => operation.source_information(),
            Self::Equality(operation) => operation.source_information(),
            Self::Order(operation) => operation.source_information(),
            Self::Pipe(operation) => operation.source_information(),
        }
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Arithmetic(operation) => operation.transform_expressions(transform)?.into(),
            Self::Boolean(operation) => operation.transform_expressions(transform)?.into(),
            Self::Equality(operation) => operation.transform_expressions(transform)?.into(),
            Self::Order(operation) => operation.transform_expressions(transform)?.into(),
            Self::Pipe(operation) => operation.transform_expressions(transform)?.into(),
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Arithmetic(operation) => operation.transform_types(transform)?.into(),
            Self::Boolean(operation) => operation.transform_types(transform)?.into(),
            Self::Equality(operation) => operation.transform_types(transform)?.into(),
            Self::Order(operation) => operation.transform_types(transform)?.into(),
            Self::Pipe(operation) => operation.transform_types(transform)?.into(),
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

impl From<EqualityOperation> for Operation {
    fn from(operation: EqualityOperation) -> Self {
        Self::Equality(operation)
    }
}

impl From<OrderOperation> for Operation {
    fn from(operation: OrderOperation) -> Self {
        Self::Order(operation)
    }
}

impl From<PipeOperation> for Operation {
    fn from(operation: PipeOperation) -> Self {
        Self::Pipe(operation)
    }
}
