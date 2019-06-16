mod function;
mod variable;

pub use function::*;
pub use variable::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Function(Function),
    Number,
    Variable(Variable),
}

impl Type {
    pub fn substitute_variable(&self, variable: &Variable, type_: &Self) -> Self {
        match self {
            Type::Function(function) => Function::new(
                function.argument().substitute_variable(variable, type_),
                function.result().substitute_variable(variable, type_),
            )
            .into(),
            Type::Number => Type::Number,
            Type::Variable(another_variable) => {
                if variable.id() == another_variable.id() {
                    type_.clone()
                } else {
                    another_variable.clone().into()
                }
            }
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
