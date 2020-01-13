use super::canonicalize::canonicalize;
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

    pub fn unwrap(&self) -> Self {
        self.unwrap_once(0, self)
    }

    pub(super) fn unwrap_once(&self, index: usize, type_: &Self) -> Self {
        Self {
            arguments: self
                .arguments
                .iter()
                .map(|argument| canonicalize(&argument.unwrap_once(index, &type_)))
                .collect(),
            result: self.result.clone(),
        }
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

    #[test]
    fn arguments() {
        assert_eq!(
            Function::new(vec![Value::Number.into()], Value::Number).arguments(),
            &[Value::Number.into()]
        );
        assert_eq!(
            Function::new(vec![Type::Index(0)], Value::Number).arguments(),
            &[Type::Index(0)]
        );
    }

    #[test]
    fn result() {
        assert_eq!(
            Function::new(vec![Value::Number.into()], Value::Number).result(),
            &Value::Number
        );
    }

    #[test]
    fn unwrap() {
        for (one, other) in &[
            (
                Function::new(vec![Value::Number.into()], Value::Number),
                Function::new(vec![Value::Number.into()], Value::Number),
            ),
            (
                Function::new(vec![Type::Index(0)], Value::Number),
                Function::new(
                    vec![Function::new(vec![Type::Index(0)], Value::Number).into()],
                    Value::Number,
                ),
            ),
            (
                Function::new(
                    vec![
                        Function::new(vec![Type::Index(1), Value::Number.into()], Value::Number)
                            .into(),
                    ],
                    Value::Number,
                ),
                Function::new(
                    vec![Function::new(
                        vec![
                            Function::new(vec![Type::Index(1)], Value::Number).into(),
                            Value::Number.into(),
                        ],
                        Value::Number,
                    )
                    .into()],
                    Value::Number,
                ),
            ),
        ] {
            assert_eq!(&one.unwrap(), other);
        }
    }
}
