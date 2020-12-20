use crate::ast::*;
use std::sync::Arc;

pub struct BooleanOperationTransformer {}

impl BooleanOperationTransformer {
    pub fn new() -> Arc<Self> {
        Self {}.into()
    }

    pub fn transform(&self, operation: &GenericOperation) -> Expression {
        let source_information = operation.source_information();

        match operation.operator() {
            Operator::And => If::new(
                operation.lhs().clone(),
                operation.rhs().clone(),
                Boolean::new(false, source_information.clone()),
                source_information.clone(),
            )
            .into(),
            Operator::Or => If::new(
                operation.lhs().clone(),
                Boolean::new(true, source_information.clone()),
                operation.rhs().clone(),
                source_information.clone(),
            )
            .into(),
            _ => operation.clone().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn transform_and_operation() {
        assert_eq!(
            BooleanOperationTransformer::new().transform(&GenericOperation::with_type(
                types::Boolean::new(SourceInformation::dummy()),
                Operator::And,
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
            BooleanOperationTransformer::new().transform(&GenericOperation::with_type(
                types::Boolean::new(SourceInformation::dummy()),
                Operator::Or,
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
