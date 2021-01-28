use super::super::error::CompileError;
use super::super::type_comparability_checker::TypeComparabilityChecker;
use super::utilities;
use crate::ast::*;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct RecordEqualFunctionTransformer {
    type_comparability_checker: Arc<TypeComparabilityChecker>,
}

impl RecordEqualFunctionTransformer {
    pub fn new(type_comparability_checker: Arc<TypeComparabilityChecker>) -> Self {
        Self {
            type_comparability_checker,
        }
    }

    pub fn transform(&mut self, module: &Module) -> Result<Module, CompileError> {
        let mut equal_function_definitions = vec![];

        for type_definition in module.type_definitions() {
            if let Type::Record(record_type) = type_definition.type_() {
                if self
                    .type_comparability_checker
                    .check(type_definition.type_())?
                {
                    equal_function_definitions.push(self.create_record_equal_function(record_type));
                }
            }
        }

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            module.export_foreign().clone(),
            module.imports().to_vec(),
            module.import_foreigns().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .cloned()
                .chain(equal_function_definitions.into_iter().map(Definition::from))
                .collect(),
        ))
    }

    fn create_record_equal_function(&mut self, record_type: &types::Record) -> FunctionDefinition {
        let source_information = record_type.source_information();
        let mut expression: Expression = Boolean::new(true, source_information.clone()).into();

        for (key, element_type) in record_type.elements() {
            expression = If::new(
                expression,
                EqualityOperation::with_type(
                    element_type.clone(),
                    EqualityOperator::Equal,
                    RecordElementOperation::new(
                        record_type.clone(),
                        key,
                        Variable::new("lhs", source_information.clone()),
                        "$element",
                        Variable::new("$element", source_information.clone()),
                        source_information.clone(),
                    ),
                    RecordElementOperation::new(
                        record_type.clone(),
                        key,
                        Variable::new("rhs", source_information.clone()),
                        "$element",
                        Variable::new("$element", source_information.clone()),
                        source_information.clone(),
                    ),
                    source_information.clone(),
                ),
                Boolean::new(false, source_information.clone()),
                source_information.clone(),
            )
            .into();
        }

        FunctionDefinition::new(
            utilities::get_record_equal_function_name(record_type),
            vec!["lhs".into(), "rhs".into()],
            expression,
            types::Function::new(
                record_type.clone(),
                types::Function::new(
                    record_type.clone(),
                    types::Boolean::new(source_information.clone()),
                    source_information.clone(),
                ),
                source_information.clone(),
            ),
            source_information.clone(),
        )
    }
}
