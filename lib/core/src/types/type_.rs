use super::function::Function;
use super::value::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Function(Function),
    Value(Value),
}

impl Type {
    pub fn to_id(&self) -> String {
        match self {
            Self::Function(function) => function.to_id(),
            Self::Value(value) => value.to_id(),
        }
    }

    pub fn into_function(self) -> Option<Function> {
        match self {
            Self::Function(function) => Some(function),
            Self::Value(_) => None,
        }
    }

    pub fn into_value(self) -> Option<Value> {
        match self {
            Self::Function(_) => None,
            Self::Value(value) => Some(value),
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
    fn to_id() {
        assert_eq!(
            &Type::from(Function::new(vec![Value::Number.into()], Value::Number)).to_id(),
            "(Number->Number)"
        );
    }
}
