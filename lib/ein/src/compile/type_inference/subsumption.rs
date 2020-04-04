use crate::types::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Subsumption {
    lower: Type,
    upper: Type,
}

impl Subsumption {
    pub fn new(lower: impl Into<Type>, upper: impl Into<Type>) -> Self {
        Self {
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    pub fn lower(&self) -> &Type {
        &self.lower
    }

    pub fn upper(&self) -> &Type {
        &self.upper
    }
}
