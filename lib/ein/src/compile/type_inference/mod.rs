mod constraint_checker;
mod constraint_collector;
mod constraint_solver;
mod subsumption_set;
mod type_inferrer;
mod variable_constraint;
mod variable_constraint_set;
mod variable_substitutor;

use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_canonicalizer::TypeCanonicalizer;
use super::type_equality_checker::TypeEqualityChecker;
use crate::ast::*;
use type_inferrer::TypeInferrer;

pub fn infer_types(module: &Module) -> Result<Module, CompileError> {
    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
    let type_canonicalizer = TypeCanonicalizer::new(
        reference_type_resolver.clone(),
        type_equality_checker.clone(),
    );

    TypeInferrer::new(
        reference_type_resolver,
        type_equality_checker,
        type_canonicalizer,
    )
    .infer(module)
}
