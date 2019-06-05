use super::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    argument: Box<Type>,
    result: Box<Type>,
}

impl Function {
    pub fn new(argument: Type, result: Type) -> Self {
        Self {
            argument: Box::new(argument),
            result: Box::new(result),
        }
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Type::Function(function)
    }
}
