use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption_set::SubsumptionSet;
use super::variable_constraint_set::VariableConstraintSet;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ConstraintSolver {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl ConstraintSolver {
    pub fn new(reference_type_resolver: Arc<ReferenceTypeResolver>) -> Self {
        Self {
            reference_type_resolver,
        }
    }

    pub fn solve(
        &self,
        mut solved_subsumption_set: SubsumptionSet,
        checked_subsumption_set: &mut SubsumptionSet,
    ) -> Result<HashMap<usize, Type>, CompileError> {
        let mut variable_constraint_set = VariableConstraintSet::new();

        while let Some(subsumption) = solved_subsumption_set.remove() {
            match subsumption {
                (lower, Type::Variable(variable)) => {
                    if let Type::Variable(lower) = &lower {
                        if variable.id() == lower.id() {
                            continue;
                        }
                    }

                    for type_ in variable_constraint_set.get_upper_types(variable.id()) {
                        solved_subsumption_set.add(lower.clone(), type_.clone());
                    }

                    variable_constraint_set.add_lower_type(&variable, &lower);
                }
                (Type::Variable(variable), upper) => {
                    for type_ in variable_constraint_set.get_lower_types(variable.id()) {
                        solved_subsumption_set.add(type_.clone(), upper.clone());
                    }

                    variable_constraint_set.add_upper_type(&variable, &upper);
                }
                (Type::Reference(reference), other) => solved_subsumption_set.add(
                    self.reference_type_resolver.resolve_reference(&reference)?,
                    other.clone(),
                ),
                (one, Type::Reference(reference)) => solved_subsumption_set.add(
                    one.clone(),
                    self.reference_type_resolver.resolve_reference(&reference)?,
                ),
                (Type::Function(one), Type::Function(other)) => {
                    solved_subsumption_set.add(other.argument().clone(), one.argument().clone());
                    solved_subsumption_set.add(one.result().clone(), other.result().clone());
                }
                (Type::List(one), Type::List(other)) => {
                    solved_subsumption_set.add(one.element().clone(), other.element().clone());
                }
                (Type::Union(union), other) => {
                    for type_ in union.types() {
                        solved_subsumption_set.add(type_.clone(), other.clone());
                    }
                }
                subsumption => checked_subsumption_set.add(subsumption.0, subsumption.1),
            }
        }

        Ok(variable_constraint_set.to_substitutions())
    }
}
