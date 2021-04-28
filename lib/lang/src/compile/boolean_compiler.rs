use super::type_compiler::TypeCompiler;
use std::sync::Arc;

pub struct BooleanCompiler {
    type_compiler: Arc<TypeCompiler>,
}

impl BooleanCompiler {
    pub fn new(type_compiler: Arc<TypeCompiler>) -> Arc<Self> {
        Self { type_compiler }.into()
    }

    pub fn compile(&self, value: bool) -> eir::ir::ConstructorApplication {
        eir::ir::ConstructorApplication::new(
            eir::ir::Constructor::new(self.type_compiler.compile_boolean(), value as u64),
            vec![],
        )
    }

    pub fn compile_conversion(
        &self,
        expression: impl Into<eir::ir::Expression>,
    ) -> eir::ir::Expression {
        eir::ir::PrimitiveCase::new(
            expression.into(),
            vec![
                eir::ir::PrimitiveAlternative::new(
                    eir::ir::Primitive::Boolean(false),
                    self.compile(false),
                ),
                eir::ir::PrimitiveAlternative::new(
                    eir::ir::Primitive::Boolean(true),
                    self.compile(true),
                ),
            ],
            None,
        )
        .into()
    }
}
