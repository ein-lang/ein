use super::function::Function;
use super::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Function(Function),
    Value(Value),
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}
