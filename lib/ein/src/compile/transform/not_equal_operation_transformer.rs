use crate::ast::*;
use std::sync::Arc;

pub struct NotEqualOperationTransformer {}

impl NotEqualOperationTransformer {
    pub fn new() -> Arc<Self> {
        Self {}.into()
    }

    pub fn transform(&self, operation: &Operation) -> Expression {
        if operation.operator() == Operator::NotEqual {
            let source_information = operation.source_information();

            If::new(
                Operation::with_type(
                    operation.type_().clone(),
                    Operator::Equal,
                    operation.lhs().clone(),
                    operation.rhs().clone(),
                    source_information.clone(),
                ),
                Boolean::new(false, source_information.clone()),
                Boolean::new(true, source_information.clone()),
                source_information.clone(),
            )
            .into()
        } else {
            operation.clone().into()
        }
    }
}
