use super::function::Function;
use super::value::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Function(Function),
    Index(usize),
    Value(Value),
}

impl Type {
    pub fn to_id(&self) -> String {
        match self {
            Self::Function(function) => function.to_id(),
            Self::Index(index) => format!("{}", index),
            Self::Value(value) => value.to_id(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_id() {
        assert_eq!(
            &Function::new(vec![Type::Index(0)], Value::Number).to_id(),
            "(0->Number)"
        );
    }
}
