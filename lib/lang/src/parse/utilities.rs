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
        ParsedOperator::Or => {
            BooleanOperation::new(BooleanOperator::Or, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::And => {
            BooleanOperation::new(BooleanOperator::And, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Equal => {
            GenericOperation::new(Operator::Equal, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::NotEqual => {
            GenericOperation::new(Operator::NotEqual, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Add => {
            GenericOperation::new(Operator::Add, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Subtract => {
            GenericOperation::new(Operator::Subtract, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Multiply => {
            GenericOperation::new(Operator::Multiply, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::Divide => {
            GenericOperation::new(Operator::Divide, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::LessThan => {
            GenericOperation::new(Operator::LessThan, lhs, rhs, source_information.clone()).into()
        }
        ParsedOperator::LessThanOrEqual => GenericOperation::new(
            Operator::LessThanOrEqual,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
        ParsedOperator::GreaterThan => {
            GenericOperation::new(Operator::GreaterThan, lhs, rhs, source_information.clone())
                .into()
        }
        ParsedOperator::GreaterThanOrEqual => GenericOperation::new(
            Operator::GreaterThanOrEqual,
            lhs,
            rhs,
            source_information.clone(),
        )
        .into(),
    }
}

fn operator_priority(operator: ParsedOperator) -> usize {
    match operator {
        ParsedOperator::Or => 0,
        ParsedOperator::And => 1,
        ParsedOperator::Equal
        | ParsedOperator::NotEqual
        | ParsedOperator::LessThan
        | ParsedOperator::LessThanOrEqual
        | ParsedOperator::GreaterThan
        | ParsedOperator::GreaterThanOrEqual => 2,
        ParsedOperator::Add | ParsedOperator::Subtract => 3,
        ParsedOperator::Multiply | ParsedOperator::Divide => 4,
    }
}
