use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;

pub struct ModuleCompiler<'a> {
    module: &'a llvm::Module,
    ast_module: &'a ast::Module,
    type_compiler: TypeCompiler,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(module: &'a llvm::Module, ast_module: &'a ast::Module) -> ModuleCompiler<'a> {
        ModuleCompiler {
            module,
            ast_module,
            type_compiler: TypeCompiler::new(),
        }
    }

    pub fn compile(&self) -> Result<(), CompileError> {
        unsafe {
            self.module.declare_intrinsics();

            for function_definition in self.ast_module.function_definitions() {
                let function = self.module.add_function(
                    if function_definition.name() == "main" {
                        "sloth_main"
                    } else {
                        function_definition.name()
                    },
                    self.type_compiler
                        .compile_function(&function_definition.type_()),
                );

                let builder = llvm::Builder::new(function);
                builder.position_at_end(builder.append_basic_block("entry"));
                builder.build_ret(
                    ExpressionCompiler::new(&builder).compile(&function_definition.body())?,
                );

                llvm::verify_function(function);
            }

            llvm::verify_module(&self.module);
        }

        Ok(())
    }
}
