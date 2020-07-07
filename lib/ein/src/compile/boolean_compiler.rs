use super::type_compiler::TypeCompiler;
use std::sync::Arc;

pub struct BooleanCompiler {
    type_compiler: Arc<TypeCompiler>,
}

impl BooleanCompiler {
    pub fn new(type_compiler: Arc<TypeCompiler>) -> Arc<Self> {
        Self { type_compiler }.into()
    }

    pub fn compile(&self, value: bool) -> ssf::ir::ConstructorApplication {
        ssf::ir::ConstructorApplication::new(
            ssf::ir::Constructor::new(self.type_compiler.compile_boolean(), value as u64),
            vec![],
        )
    }

    pub fn compile_conversion(
        &self,
        expression: impl Into<ssf::ir::Expression>,
    ) -> ssf::ir::Expression {
        ssf::ir::PrimitiveCase::new(
            expression.into(),
            vec![
                ssf::ir::PrimitiveAlternative::new(
                    ssf::ir::Primitive::Integer8(0),
                    self.compile(false),
                ),
                ssf::ir::PrimitiveAlternative::new(
                    ssf::ir::Primitive::Integer8(1),
                    self.compile(true),
                ),
            ],
            None,
        )
        .into()
    }
}
