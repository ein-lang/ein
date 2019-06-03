use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::llvm;

pub struct ModuleCompiler<'a> {
    module: &'a llvm::Module,
    expression: &'a ast::Expression,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(module: &'a llvm::Module, expression: &'a ast::Expression) -> ModuleCompiler<'a> {
        ModuleCompiler { module, expression }
    }

    pub fn compile(&self) -> Result<(), CompileError> {
        unsafe {
            self.module.declare_intrinsics();

            let function = self.module.add_function(
                "sloth_main",
                llvm::function_type(llvm::double_type(), &mut []),
            );

            let builder = llvm::Builder::new(function);
            builder.position_at_end(builder.append_basic_block("entry"));
            builder.build_ret(ExpressionCompiler::new(&builder).compile(&self.expression)?);

            llvm::verify_function(function);
            llvm::verify_module(&self.module);
        }

        Ok(())
    }
}
