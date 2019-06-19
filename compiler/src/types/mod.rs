mod function;
mod variable;

pub use function::*;
use std::collections::HashMap;
pub use variable::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Function(Function),
    Number,
    Variable(Variable),
}

impl Type {
    pub fn substitute_variable(&self, variable: &Variable, type_: &Self) -> Self {
        self.substitute_variables(&vec![(variable.id(), type_.clone())].into_iter().collect())
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        match self {
            Type::Function(function) => function.substitute_variables(substitutions).into(),
            Type::Number => Type::Number,
            Type::Variable(variable) => variable.substitute_variables(substitutions),
        }
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Type::Function(function)
    }
}

impl From<Variable> for Type {
    fn from(variable: Variable) -> Self {
        Type::Variable(variable)
    }
}
