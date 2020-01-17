use crate::types::{Type, Value};

pub struct TypeEqualityChecker<'a> {
    pairs: Vec<(&'a Type, &'a Type)>,
}

impl<'a> TypeEqualityChecker<'a> {
    pub fn new(types: &'a [&'a Type]) -> Self {
        Self {
            pairs: types.iter().cloned().zip(types.iter().cloned()).collect(),
        }
    }

    // equal checks if two types are equivalent structurally even when they are
    // represented differently.
    pub fn equal(&self, one: &Type, other: &Type) -> bool {
        match (one, other) {
            (Type::Value(Value::Number), Type::Value(Value::Number)) => true,
            (Type::Index(index), other) => self.equal(self.pairs[*index].0, other),
            (other, Type::Index(index)) => self.equal(other, self.pairs[*index].1),
            (Type::Function(one_function), Type::Function(other_function)) => {
                if one_function.arguments().len() != other_function.arguments().len() {
                    return false;
                } else if self.pairs.contains(&(one, other)) {
                    return true;
                }

                let checker = self.push_pair(one, other);

                one_function
                    .arguments()
                    .iter()
                    .zip(other_function.arguments())
                    .all(|(one, other)| checker.equal(one, other))
                    && checker.equal_values(one_function.result(), other_function.result())
            }
            (_, _) => false,
        }
    }

    fn equal_values(&self, one: &Value, other: &Value) -> bool {
        match (one, other) {
            (Value::Number, Value::Number) => true,
        }
    }

    fn push_pair(&'a self, one: &'a Type, other: &'a Type) -> Self {
        Self {
            pairs: [(one, other)]
                .iter()
                .chain(self.pairs.iter())
                .copied()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Function;

    #[test]
    fn equal() {
        for (one, other) in &[
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
            assert!(TypeEqualityChecker::new(&[]).equal(one, other));
        }

        for (one, other) in &[
            (
                Value::Number.into(),
                Function::new(vec![Value::Number.into()], Value::Number.into()).into(),
            ),
            (
                Function::new(
                    vec![Value::Number.into(), Value::Number.into()],
                    Value::Number.into(),
                )
                .into(),
                Function::new(vec![Value::Number.into()], Value::Number.into()).into(),
            ),
            (
                Function::new(
                    vec![Function::new(
                        vec![
                            Function::new(vec![Type::Index(0)], Value::Number.into()).into(),
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
            assert!(!TypeEqualityChecker::new(&[]).equal(one, other));
        }
    }
}
