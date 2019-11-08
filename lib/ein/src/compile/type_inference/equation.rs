use crate::types::*;

#[derive(Clone, Debug)]
pub struct Equation {
    lhs: Type,
    rhs: Type,
}

impl Equation {
    pub fn new(lhs: impl Into<Type>, rhs: impl Into<Type>) -> Self {
        Self {
            lhs: lhs.into(),
            rhs: rhs.into(),
        }
    }

    pub fn lhs(&self) -> &Type {
        &self.lhs
    }

    pub fn rhs(&self) -> &Type {
        &self.rhs
    }
}
