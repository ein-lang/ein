mod function;

pub use function::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Function(Function),
    Number,
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Type::Function(function)
    }
}
