mod constraint_checker;
mod constraint_collector;
mod constraint_converter;
mod constraint_solver;
mod subsumption_set;
mod type_inferrer;
mod variable_constraint;
mod variable_constraint_set;
mod variable_substitutor;

use super::builtin_configuration::BuiltinConfiguration;
use super::error::CompileError;
use super::module_environment_creator::ModuleEnvironmentCreator;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_canonicalizer::TypeCanonicalizer;
use super::type_equality_checker::TypeEqualityChecker;
use crate::ast::*;
use constraint_collector::ConstraintCollector;
use constraint_converter::ConstraintConverter;
use constraint_solver::ConstraintSolver;
use std::sync::Arc;
use type_inferrer::TypeInferrer;

pub fn infer_types(
    module: &Module,
    builtin_configuration: Arc<BuiltinConfiguration>,
) -> Result<Module, CompileError> {
    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let type_canonicalizer = TypeCanonicalizer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    );
    let constraint_converter = ConstraintConverter::new(reference_type_resolver.clone());
    let module_environment_creator = ModuleEnvironmentCreator::new(builtin_configuration);
    let constraint_collector =
        ConstraintCollector::new(reference_type_resolver.clone(), module_environment_creator);
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
