use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;
use std::collections::HashMap;

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

            for definition in self.ast_module.definitions() {
                match definition {
                    ast::Definition::FunctionDefinition(function_definition) => {
                        self.compile_function_definition(function_definition)?
                    }
                    ast::Definition::VariableDefinition(_variable_definition) => unimplemented!(),
                }
            }

            llvm::verify_module(&self.module);
        }

        Ok(())
    }

    unsafe fn compile_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<(), CompileError> {
        let function = self.module.add_function(
            if function_definition.name() == "main" {
                "sloth_main"
            } else {
                function_definition.name()
            },
            self.type_compiler
                .compile_function(&function_definition.type_()),
        );

        let mut arguments = HashMap::new();

        for (index, name) in function_definition.arguments().iter().enumerate() {
            arguments.insert(name.clone(), llvm::get_param(function, index as u32));
        }

        let builder = llvm::Builder::new(function);
        builder.position_at_end(builder.append_basic_block("entry"));
        builder.build_ret(
            ExpressionCompiler::new(&builder, &arguments).compile(&function_definition.body())?,
        );

        llvm::verify_function(function);
        Ok(())
    }
}
