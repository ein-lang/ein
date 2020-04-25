use super::variable_constraint::VariableConstraint;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct VariableConstraintSet {
    constraints: HashMap<usize, VariableConstraint>,
}

impl VariableConstraintSet {
    pub fn new() -> Self {
        Self {
            constraints: Default::default(),
        }
    }

    pub fn get_lower_types(&self, id: usize) -> impl Iterator<Item = &Type> {
        self.constraints
            .get(&id)
            .map(|constraint| constraint.lower_types())
            .into_iter()
            .flatten()
    }

    pub fn get_upper_types(&mut self, id: usize) -> impl Iterator<Item = &Type> {
        self.constraints
            .get(&id)
            .map(|constraint| constraint.upper_types())
            .into_iter()
            .flatten()
    }

    pub fn add_lower_type(&mut self, variable: &types::Variable, type_: &Type) {
        self.get_constraint(variable).add_lower_type(type_)
    }

    pub fn add_upper_type(&mut self, variable: &types::Variable, type_: &Type) {
        self.get_constraint(variable).add_upper_type(type_)
    }

    fn get_constraint(&mut self, variable: &types::Variable) -> &mut VariableConstraint {
        let id = variable.id();

        if self.constraints.get(&id).is_none() {
            self.constraints.insert(
                id,
                VariableConstraint::new(variable.source_information().clone()),
            );
        }

        self.constraints.get_mut(&id).unwrap()
    }

    pub fn to_substitutions(&self) -> HashMap<usize, Type> {
        let mut substitutions = HashMap::<usize, Type>::new();

        for (id, constraint) in &self.constraints {
            let constraint_type = constraint.to_type().substitute_variables(&substitutions);

            for type_ in substitutions.values_mut() {
                *type_ = type_.substitute_variable(*id, &constraint_type);
            }

            substitutions.insert(*id, constraint_type);
        }

        substitutions
    }
}
