mod constraint_checker;
mod constraint_collector;
mod constraint_converter;
mod constraint_solver;
mod subsumption_set;
mod type_inferrer;
mod variable_constraint;
mod variable_constraint_set;
mod variable_substitutor;

use super::{
    compile_configuration::CompileConfiguration, error::CompileError,
    module_environment_creator::ModuleEnvironmentCreator,
    reference_type_resolver::ReferenceTypeResolver, type_canonicalizer::TypeCanonicalizer,
    type_equality_checker::TypeEqualityChecker,
};
use crate::ast::*;
use constraint_collector::ConstraintCollector;
use constraint_converter::ConstraintConverter;
use constraint_solver::ConstraintSolver;
use std::sync::Arc;
use type_inferrer::TypeInferrer;

pub fn infer_types(
    module: &Module,
    compile_configuration: Arc<CompileConfiguration>,
) -> Result<Module, CompileError> {
    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let type_canonicalizer = TypeCanonicalizer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    );
    let constraint_converter = ConstraintConverter::new(reference_type_resolver.clone());
    let module_environment_creator = ModuleEnvironmentCreator::new();
    let constraint_collector = ConstraintCollector::new(
        reference_type_resolver.clone(),
        module_environment_creator,
        compile_configuration.error_type_configuration.clone(),
    );
    let constraint_solver =
        ConstraintSolver::new(constraint_converter, reference_type_resolver.clone());

    TypeInferrer::new(
        reference_type_resolver,
        type_equality_checker,
        type_canonicalizer,
        constraint_collector,
        constraint_solver,
    )
    .infer(module)
}
