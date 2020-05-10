use crate::ast::*;
use crate::debug::SourceInformation;
use std::collections::VecDeque;

pub fn reduce_operations(
    lhs: Expression,
    mut pairs: VecDeque<(Operator, Expression, SourceInformation)>,
) -> Operation {
    let (operator, rhs, source_information) = pairs.pop_front().unwrap();

    if pairs.is_empty() {
        Operation::new(operator, lhs, rhs, source_information)
    } else if operator_priority(operator) >= operator_priority(pairs[0].0) {
        reduce_operations(
            Expression::Operation(Operation::new(operator, lhs, rhs, source_information)),
            pairs,
        )
    } else {
        Operation::new(
            operator,
            lhs,
            reduce_operations(rhs, pairs),
            source_information,
        )
    }
}

fn operator_priority(operator: Operator) -> u8 {
    match operator {
        Operator::Or => 0,
        Operator::And => 1,
        Operator::Equal
        | Operator::NotEqual
        | Operator::LessThan
        | Operator::LessThanOrEqual
        | Operator::GreaterThan
        | Operator::GreaterThanOrEqual => 2,
        Operator::Add | Operator::Subtract => 3,
        Operator::Multiply | Operator::Divide => 4,
    }
}
