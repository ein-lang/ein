use super::type_::Type;
use super::value::Value;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    arguments: Vec<Type>,
    result: Rc<Value>,
}

impl Function {
    pub fn new(arguments: Vec<Type>, result: Value) -> Self {
        Self {
            arguments,
            result: Rc::new(result),
        }
    }

    pub fn arguments(&self) -> &[Type] {
        &self.arguments
    }

    pub fn result(&self) -> &Value {
        &self.result
    }
}
