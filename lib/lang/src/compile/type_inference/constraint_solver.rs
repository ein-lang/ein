use super::{
    super::{error::CompileError, reference_type_resolver::ReferenceTypeResolver},
    constraint_converter::ConstraintConverter,
    subsumption_set::SubsumptionSet,
    variable_constraint_set::VariableConstraintSet,
};
use crate::types::Type;
use std::{collections::HashMap, sync::Arc};

pub struct ConstraintSolver {
    constraint_converter: Arc<ConstraintConverter>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl ConstraintSolver {
    pub fn new(
        constraint_converter: Arc<ConstraintConverter>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
    ) -> Arc<Self> {
        Self {
            constraint_converter,
            reference_type_resolver,
        }
        .into()
    }

    pub fn solve(
        &self,
        mut solved_subsumption_set: SubsumptionSet,
        checked_subsumption_set: &mut SubsumptionSet,
    ) -> Result<HashMap<usize, Type>, CompileError> {
        let mut constraint_set = VariableConstraintSet::new();

        while let Some(subsumption) = solved_subsumption_set.remove() {
            match subsumption {
                (lower, Type::Variable(variable)) => {
                    if let Type::Variable(lower) = &lower {
                        if variable.id() == lower.id() {
                            continue;
                        }
                    }

                    for type_ in constraint_set.get_upper_types(variable.id()) {
                        solved_subsumption_set.add(lower.clone(), type_.clone());
                    }

                    constraint_set.add_lower_type(&variable, &lower);
                }
                (Type::Variable(variable), upper) => {
                    for type_ in constraint_set.get_lower_types(variable.id()) {
                        solved_subsumption_set.add(type_.clone(), upper.clone());
                    }

                    constraint_set.add_upper_type(&variable, &upper);
                }
                (Type::Reference(reference), upper) => solved_subsumption_set.add(
                    self.reference_type_resolver.resolve_reference(&reference)?,
                    upper.clone(),
                ),
                (lower, Type::Reference(reference)) => solved_subsumption_set.add(
                    lower.clone(),
                    self.reference_type_resolver.resolve_reference(&reference)?,
                ),
                (Type::Function(one), Type::Function(other)) => {
                    solved_subsumption_set.add(other.argument().clone(), one.argument().clone());
                    checked_subsumption_set.add(one.argument().clone(), other.argument().clone());
                    solved_subsumption_set.add(one.result().clone(), other.result().clone());
                    checked_subsumption_set.add(other.result().clone(), one.result().clone());
                }
                (Type::List(one), Type::List(other)) => {
                    solved_subsumption_set.add(one.element().clone(), other.element().clone());
                }
                subsumption => checked_subsumption_set.add(subsumption.0, subsumption.1),
            }
        }

        self.convert_to_substitutions(&constraint_set)
    }

    fn convert_to_substitutions(
        &self,
        constraint_set: &VariableConstraintSet,
    ) -> Result<HashMap<usize, Type>, CompileError> {
        let mut substitutions = HashMap::<usize, Type>::new();

        for (id, constraint) in constraint_set.constraints() {
            let constraint_type = self
                .constraint_converter
                .convert(constraint)?
                .substitute_variables(&substitutions);

            for type_ in substitutions.values_mut() {
                *type_ = type_.substitute_variable(*id, &constraint_type);
            }

            substitutions.insert(*id, constraint_type);
        }

        Ok(substitutions)
    }
}
