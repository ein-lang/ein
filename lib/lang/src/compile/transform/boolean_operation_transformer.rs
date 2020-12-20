use crate::ast::*;
use std::sync::Arc;

pub struct BooleanOperationTransformer {}

impl BooleanOperationTransformer {
    pub fn new() -> Arc<Self> {
        Self {}.into()
    }

    pub fn transform(&self, operation: &BooleanOperation) -> Expression {
        let source_information = operation.source_information();

        match operation.operator() {
            BooleanOperator::And => If::new(
                operation.lhs().clone(),
                operation.rhs().clone(),
                Boolean::new(false, source_information.clone()),
                source_information.clone(),
            )
            .into(),
            BooleanOperator::Or => If::new(
                operation.lhs().clone(),
                Boolean::new(true, source_information.clone()),
                operation.rhs().clone(),
                source_information.clone(),
            )
            .into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use pretty_assertions::assert_eq;

    #[test]
    fn transform_and_operation() {
        assert_eq!(
            BooleanOperationTransformer::new().transform(&BooleanOperation::new(
                BooleanOperator::And,
                Boolean::new(true, SourceInformation::dummy()),
                Boolean::new(true, SourceInformation::dummy()),
                SourceInformation::dummy(),
            )),
            If::new(
                Boolean::new(true, SourceInformation::dummy()),
                Boolean::new(true, SourceInformation::dummy()),
                Boolean::new(false, SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
        );
    }

    #[test]
    fn transform_or_operation() {
        assert_eq!(
            BooleanOperationTransformer::new().transform(&BooleanOperation::new(
                BooleanOperator::Or,
                Boolean::new(false, SourceInformation::dummy()),
                Boolean::new(false, SourceInformation::dummy()),
                SourceInformation::dummy(),
            )),
            If::new(
                Boolean::new(false, SourceInformation::dummy()),
                Boolean::new(true, SourceInformation::dummy()),
                Boolean::new(false, SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
        );
    }
}
