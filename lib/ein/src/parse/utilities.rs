use crate::ast::*;
use crate::debug::SourceInformation;

pub fn reduce_operations(
    lhs: Expression,
    pairs: &[(Operator, Expression, SourceInformation)],
) -> Operation {
    match pairs {
        [] => unreachable!(),
        [(operator, rhs, source_information)] => {
            Operation::new(*operator, lhs, rhs.clone(), source_information.clone())
        }
        [(operator, rhs, source_information), (next_opeartor, _, _), ..] => {
            if operator_priority(*operator) >= operator_priority(*next_opeartor) {
                reduce_operations(
                    Operation::new(*operator, lhs, rhs.clone(), source_information.clone()).into(),
                    &pairs[1..],
                )
            } else {
                Operation::new(
                    *operator,
                    lhs,
                    reduce_operations(rhs.clone(), &pairs[1..]),
                    source_information.clone(),
                )
            }
        }
    }
}

fn operator_priority(operator: Operator) -> usize {
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
