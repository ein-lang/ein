mod function;
mod number;
mod variable;

use crate::debug::SourceInformation;
pub use function::*;
pub use number::*;
use std::collections::HashMap;
use std::rc::Rc;
pub use variable::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Function(Function),
    Number(Number),
    Variable(Variable),
}

impl Type {
    pub fn source_information(&self) -> &Rc<SourceInformation> {
        match self {
            Type::Function(function) => function.source_information(),
            Type::Number(number) => number.source_information(),
            Type::Variable(variable) => variable.source_information(),
        }
    }

    pub fn substitute_variable(&self, variable: &Variable, type_: &Self) -> Self {
        self.substitute_variables(&vec![(variable.id(), type_.clone())].into_iter().collect())
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Type::Function(function) => function.substitute_variables(substitutions).into(),
            Type::Number(_) => self.clone(),
            Type::Variable(variable) => match substitutions.get(&variable.id()) {
                Some(type_) => type_.clone(),
                None => self.clone(),
            },
        }
    }

    pub fn to_function(&self) -> Option<&Function> {
        if let Type::Function(function) = self {
            Some(&function)
        } else {
            None
        }
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Type::Function(function)
    }
}

impl From<Number> for Type {
    fn from(number: Number) -> Self {
        Type::Number(number)
    }
}

impl From<Variable> for Type {
    fn from(variable: Variable) -> Self {
        Type::Variable(variable)
    }
}
