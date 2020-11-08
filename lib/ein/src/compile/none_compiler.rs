use super::type_compiler::TypeCompiler;
use std::sync::Arc;

pub struct NoneCompiler {
    type_compiler: Arc<TypeCompiler>,
}

impl NoneCompiler {
    pub fn new(type_compiler: Arc<TypeCompiler>) -> Arc<Self> {
        Self { type_compiler }.into()
    }

    pub fn compile(&self) -> ssf::ir::ConstructorApplication {
        ssf::ir::ConstructorApplication::new(
            ssf::ir::Constructor::new(self.type_compiler.compile_none(), 0),
            vec![],
        )
    }
}
