use crate::ast::*;
use crate::types::Type;

pub struct UnionTypeSimplifier {}

impl UnionTypeSimplifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn simplify(&self, module: &Module) -> Module {
        module.convert_types(&mut |type_| match type_ {
            Type::Union(union) => union.simplify(),
            _ => type_.clone(),
        })
    }
}
