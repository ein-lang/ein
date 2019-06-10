use crate::ast::*;
use std::collections::VecDeque;

pub fn reduce_operations(
    lhs: Expression,
    mut pairs: VecDeque<(Operator, Expression)>,
) -> Operation {
    if pairs.len() == 0 {
        panic!("foo");
    }

    let pair = pairs.pop_front().unwrap();

    if pairs.len() == 0 {
        Operation::new(pair.0, lhs, pair.1)
    } else if operator_priority(pair.0) > operator_priority(pairs[0].0) {
        reduce_operations(
            Expression::Operation(Operation::new(pair.0, lhs, pair.1)),
            pairs,
        )
    } else {
        Operation::new(pair.0, lhs, reduce_operations(pair.1, pairs).into())
    }
}

fn operator_priority(operator: Operator) -> u8 {
    match operator {
        Operator::Add => 1,
        Operator::Subtract => 1,
        Operator::Multiply => 2,
        Operator::Divide => 2,
    }
}
