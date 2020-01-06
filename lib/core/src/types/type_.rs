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

    pub fn into_function(self) -> Option<Function> {
        match self {
            Self::Function(function) => Some(function),
            Self::Index(_) => None,
            Self::Value(_) => None,
        }
    }

    pub fn into_value(self) -> Option<Value> {
        match self {
            Self::Function(_) => None,
            Self::Index(_) => None,
            Self::Value(value) => Some(value),
        }
    }

    pub fn unwrap_once(&self, index: usize, type_: &Function) -> Self {
        match self {
            Self::Function(function) => function.unwrap_once(index + 1, type_).into(),
            Self::Index(current_index) => {
                if *current_index == index {
                    type_.clone().into()
                } else {
                    self.clone()
                }
            }
            Self::Value(value) => value.unwrap_once(index, type_).into(),
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
