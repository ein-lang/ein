use super::type_equality_checker::TypeEqualityChecker;
use crate::types::{Function, Type, Value};

pub struct TypeCanonicalizer<'a> {
    types: Vec<&'a Type>,
}

impl<'a> TypeCanonicalizer<'a> {
    pub fn new() -> Self {
        Self { types: vec![] }
    }

    pub fn canonicalize(&self, type_: &Type) -> Type {
        match type_ {
            Type::Value(Value::Number) => type_.clone(),
            Type::Function(function) => {
                for (index, parent_type) in self.types.iter().enumerate() {
                    if TypeEqualityChecker::new(&self.types).equal(type_, parent_type) {
                        return Type::Index(index);
                    }
                }

                let other = self.push_type(type_);

                Function::new(
                    function
                        .arguments()
                        .iter()
                        .map(|argument| other.canonicalize(argument))
                        .collect(),
                    function.result().clone(),
                )
                .into()
            }
            Type::Index(_) => type_.clone(),
        }
    }

    fn push_type(&'a self, type_: &'a Type) -> Self {
        Self {
            types: [type_].iter().chain(&self.types).cloned().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalize() {
        for (type_, canonical_type) in &[
            (Value::Number.into(), Value::Number.into()),
            (
                Function::new(vec![Value::Number.into()], Value::Number.into()).into(),
                Function::new(vec![Value::Number.into()], Value::Number.into()).into(),
            ),
            (
                Function::new(
                    vec![Function::new(vec![Type::Index(0)], Value::Number.into()).into()],
                    Value::Number.into(),
                )
                .into(),
                Function::new(vec![Type::Index(0)], Value::Number.into()).into(),
            ),
            (
                Function::new(
                    vec![Function::new(vec![Type::Index(1)], Value::Number.into()).into()],
                    Value::Number.into(),
                )
                .into(),
                Function::new(vec![Type::Index(0)], Value::Number.into()).into(),
            ),
            (
                Function::new(
                    vec![Function::new(
                        vec![
                            Function::new(vec![Type::Index(1)], Value::Number.into()).into(),
                            Value::Number.into(),
                        ],
                        Value::Number.into(),
                    )
                    .into()],
                    Value::Number.into(),
                )
                .into(),
                Function::new(
                    vec![Function::new(
                        vec![Type::Index(1), Value::Number.into()],
                        Value::Number.into(),
                    )
                    .into()],
                    Value::Number.into(),
                )
                .into(),
            ),
        ] {
            assert_eq!(
                &TypeCanonicalizer::new().canonicalize(type_),
                canonical_type
            );
        }
    }
}
