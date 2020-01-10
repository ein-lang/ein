use super::function::Function;
use super::number::Number;
use super::reference::Reference;
use super::unknown::Unknown;
use super::variable::Variable;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Type {
    Function(Function),
    Number(Number),
    Reference(Reference),
    Unknown(Unknown),
    Variable(Variable),
}

impl Type {
    pub fn source_information(&self) -> &Rc<SourceInformation> {
        match self {
            Self::Function(function) => function.source_information(),
            Self::Number(number) => number.source_information(),
            Self::Reference(reference) => reference.source_information(),
            Self::Unknown(unknown) => unknown.source_information(),
            Self::Variable(variable) => variable.source_information(),
        }
    }

    pub fn substitute_variable(&self, variable: &Variable, type_: &Self) -> Self {
        self.substitute_variables(&vec![(variable.id(), type_.clone())].into_iter().collect())
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Self::Function(function) => function.substitute_variables(substitutions).into(),
            Self::Number(_) | Self::Reference(_) | Self::Unknown(_) => self.clone(),
            Self::Variable(variable) => match substitutions.get(&variable.id()) {
                Some(type_) => type_.clone(),
                None => self.clone(),
            },
        }
    }

    pub fn resolve_reference_types(&self, environment: &HashMap<String, Type>) -> Self {
        match self {
            Self::Function(function) => function.resolve_reference_types(environment).into(),
            Self::Number(_) => self.clone(),
            Self::Reference(reference) => environment[reference.name()].clone(),
            Self::Unknown(_) | Self::Variable(_) => unreachable!(),
        }
    }

    pub fn to_function(&self) -> Option<&Function> {
        if let Self::Function(function) = self {
            Some(&function)
        } else {
            None
        }
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl From<Number> for Type {
    fn from(number: Number) -> Self {
        Self::Number(number)
    }
}

impl From<Reference> for Type {
    fn from(reference: Reference) -> Self {
        Self::Reference(reference)
    }
}

impl From<Unknown> for Type {
    fn from(unknown: Unknown) -> Self {
        Self::Unknown(unknown)
    }
}

impl From<Variable> for Type {
    fn from(variable: Variable) -> Self {
        Self::Variable(variable)
    }
}
