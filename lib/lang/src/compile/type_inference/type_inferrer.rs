use super::{
    super::{
        error::CompileError, reference_type_resolver::ReferenceTypeResolver,
        type_canonicalizer::TypeCanonicalizer, type_equality_checker::TypeEqualityChecker,
    },
    constraint_checker::ConstraintChecker,
    constraint_collector::ConstraintCollector,
    constraint_solver::ConstraintSolver,
    variable_substitutor::VariableSubstitutor,
};
use crate::{
    ast::*,
    types::{self, Type},
};
use std::sync::Arc;

pub struct TypeInferrer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
    constraint_collector: ConstraintCollector,
    constraint_solver: Arc<ConstraintSolver>,
}

impl TypeInferrer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
        constraint_collector: ConstraintCollector,
        constraint_solver: Arc<ConstraintSolver>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            type_canonicalizer,
            constraint_collector,
            constraint_solver,
        }
    }

    pub fn infer(self, module: &Module) -> Result<Module, CompileError> {
        let module = module.transform_types(&mut |type_| -> Result<_, CompileError> {
            Ok(match type_ {
                Type::Unknown(unknown) => {
                    types::Variable::new(unknown.source_information().clone()).into()
                }
                _ => type_.clone(),
            })
        })?;

        let (solved_subsumption_set, mut checked_subsumption_set) =
            self.constraint_collector.collect(&module)?;

        let substitutions = self
            .constraint_solver
            .solve(solved_subsumption_set, &mut checked_subsumption_set)?;

        let substitutor = VariableSubstitutor::new(self.type_canonicalizer.clone(), substitutions);

        let checker = ConstraintChecker::new(
            substitutor.clone(),
            self.reference_type_resolver.clone(),
            self.type_equality_checker,
        );

        checker.check(checked_subsumption_set)?;

        module.transform_types(&mut |type_| substitutor.substitute(type_))
    }
}
