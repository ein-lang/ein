use crate::ast::*;
use crate::debug::SourceInformation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParsedOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Pipe,
}

pub fn reduce_operations(
    lhs: Expression,
    pairs: &[(ParsedOperator, Expression, SourceInformation)],
) -> Expression {
    match pairs {
        [] => lhs,
        [(operator, rhs, source_information)] => {
            create_operation(*operator, lhs, rhs.clone(), source_information).into()
        }
        [(operator, rhs, source_information), (next_operator, _, _), ..] => {
            if operator_priority(*operator) >= operator_priority(*next_operator) {
                reduce_operations(
                    create_operation(*operator, lhs, rhs.clone(), source_information).into(),
                    &pairs[1..],
                )
            } else {
                create_operation(
                    *operator,
                    lhs,
                    reduce_operations(rhs.clone(), &pairs[1..]),
                    source_information,
                )
                .into()
            }
        }
    }
}

fn create_operation(
    operator: ParsedOperator,
    lhs: impl Into<Expression>,
    rhs: impl Into<Expression>,
    source_information: &SourceInformation,
) -> Operation {
    match operator {
        ParsedOperator::Pipe => PipeOperation::new(lhs, rhs, source_information.clone()).into(),
        ParsedOperator::Or => {
            BooleanOperation::new(BooleanOperator::Or, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::And => {
            BooleanOperation::new(BooleanOperator::And, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Equal => EqualityOperation::new(
            EqualityOperator::Equal,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::NotEqual => EqualityOperation::new(
            EqualityOperator::NotEqual,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::Add => ArithmeticOperation::new(
            ArithmeticOperator::Add,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::Subtract => ArithmeticOperation::new(
            ArithmeticOperator::Subtract,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::Multiply => ArithmeticOperation::new(
            ArithmeticOperator::Multiply,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::Divide => ArithmeticOperation::new(
            ArithmeticOperator::Divide,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::LessThan => OrderOperation::new(
            OrderOperator::LessThan,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::LessThanOrEqual => OrderOperation::new(
            OrderOperator::LessThanOrEqual,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::GreaterThan => OrderOperation::new(
            OrderOperator::GreaterThan,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::GreaterThanOrEqual => OrderOperation::new(
            OrderOperator::GreaterThanOrEqual,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
    }
}

fn operator_priority(operator: ParsedOperator) -> usize {
    match operator {
        ParsedOperator::Pipe => 0,
        ParsedOperator::Or => 1,
        ParsedOperator::And => 2,
        ParsedOperator::Equal
        | ParsedOperator::NotEqual
        | ParsedOperator::LessThan
        | ParsedOperator::LessThanOrEqual
        | ParsedOperator::GreaterThan
        | ParsedOperator::GreaterThanOrEqual => 3,
        ParsedOperator::Add | ParsedOperator::Subtract => 4,
        ParsedOperator::Multiply | ParsedOperator::Divide => 5,
    }
}
