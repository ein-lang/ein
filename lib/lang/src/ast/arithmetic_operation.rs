use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<ArithmeticOperator> for ssf::ir::PrimitiveOperator {
    fn from(operator: ArithmeticOperator) -> Self {
        match operator {
            ArithmeticOperator::Add => ssf::ir::PrimitiveOperator::Add,
            ArithmeticOperator::Subtract => ssf::ir::PrimitiveOperator::Subtract,
            ArithmeticOperator::Multiply => ssf::ir::PrimitiveOperator::Multiply,
            ArithmeticOperator::Divide => ssf::ir::PrimitiveOperator::Divide,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArithmeticOperation {
    operator: ArithmeticOperator,
    lhs: Arc<Expression>,
    rhs: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl ArithmeticOperation {
    pub fn new(
        operator: ArithmeticOperator,
        lhs: impl Into<Expression>,
        rhs: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            operator,
            lhs: lhs.into().into(),
            rhs: rhs.into().into(),
            source_information: source_information.into(),
        }
    }

    pub fn operator(&self) -> ArithmeticOperator {
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
        Ok(Self::new(
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
        Ok(Self::new(
            self.operator,
            self.lhs.transform_types(transform)?,
            self.rhs.transform_types(transform)?,
            self.source_information.clone(),
        ))
    }
}
