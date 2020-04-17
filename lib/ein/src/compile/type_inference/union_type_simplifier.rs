use crate::ast::*;

pub struct UnionTypeSimplifier {}

impl UnionTypeSimplifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn simplify(&self, module: &Module) -> Module {
        module.convert_types(&mut |type_| type_.simplify())
    }
}
