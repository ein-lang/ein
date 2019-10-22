use crate::types::*;

#[derive(Clone, Debug)]
pub struct Equation {
    lhs: Type,
    rhs: Type,
}

impl Equation {
    pub fn new(lhs: Type, rhs: Type) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &Type {
        &self.lhs
    }

    pub fn rhs(&self) -> &Type {
        &self.rhs
    }
}
