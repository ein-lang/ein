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
            self.declare_intrinsics();

            let function = self.module.add_function(
                "sloth_main",
                llvm::function_type(llvm::double_type(), &mut []),
            );

            let builder = llvm::Builder::new(&self.module);
            builder.position_at_end(builder.append_basic_block(function, "entry"));
            builder.build_ret(ExpressionCompiler::new(&self.module).compile(&self.expression)?);

            llvm::verify_function(function);
            llvm::verify_module(&self.module);
        }

        Ok(())
    }

    unsafe fn declare_intrinsics(&self) {
        self.declare_function(
            "llvm.coro.id",
            llvm::token_type(),
            &mut [
                llvm::i32_type(),
                llvm::generic_pointer_type(),
                llvm::generic_pointer_type(),
                llvm::generic_pointer_type(),
            ],
        );

        self.declare_function("llvm.coro.size.i32", llvm::i32_type(), &mut []);
        self.declare_function("llvm.coro.size.i64", llvm::i64_type(), &mut []);

        self.declare_function(
            "llvm.coro.begin",
            llvm::generic_pointer_type(),
            &mut [llvm::token_type(), llvm::generic_pointer_type()],
        );
        self.declare_function(
            "llvm.coro.end",
            llvm::i1_type(),
            &mut [llvm::generic_pointer_type(), llvm::i1_type()],
        );
        self.declare_function(
            "llvm.coro.suspend",
            llvm::i8_type(),
            &mut [llvm::token_type(), llvm::i1_type()],
        );
        self.declare_function(
            "llvm.coro.free",
            llvm::generic_pointer_type(),
            &mut [llvm::token_type(), llvm::generic_pointer_type()],
        );

        self.declare_function(
            "malloc",
            llvm::generic_pointer_type(),
            &mut [llvm::i32_type()],
        );
    }

    unsafe fn declare_function(
        &self,
        name: &str,
        return_type: llvm::Type,
        arguments: &mut [llvm::Type],
    ) {
        self.module
            .add_function(name, llvm::function_type(return_type, arguments));
    }
}
