use super::type_::Type;
use super::value::Value;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    pub fn to_id(&self) -> String {
        format!(
            "({}->{})",
            self.arguments
                .iter()
                .map(|argument| argument.to_id())
                .collect::<Vec<_>>()
                .join("->"),
            self.result.to_id()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_id() {
        assert_eq!(
            &Function::new(vec![Value::Number.into()], Value::Number).to_id(),
            "(Number->Number)"
        );
        assert_eq!(
            &Function::new(
                vec![Value::Number.into(), Value::Number.into()],
                Value::Number
            )
            .to_id(),
            "(Number->Number->Number)"
        );
        assert_eq!(
            &Function::new(
                vec![Function::new(vec![Value::Number.into()], Value::Number).into()],
                Value::Number
            )
            .to_id(),
            "((Number->Number)->Number)"
        );
    }
}
