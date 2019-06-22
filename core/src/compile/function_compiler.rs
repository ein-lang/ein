use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct FunctionCompiler<'a> {
    module: &'a llvm::Module,
    type_compiler: &'a TypeCompiler,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(module: &'a llvm::Module, type_compiler: &'a TypeCompiler) -> Self {
        Self {
            module,
            type_compiler,
        }
    }

    pub unsafe fn compile(
        &self,
        function_definition: &ast::FunctionDefinition,
        variables: &HashMap<String, llvm::Value>,
    ) -> Result<llvm::Value, CompileError> {
        let entry_function = self.module.add_function(
            &Self::generate_closure_entry_name(function_definition.name()),
            self.type_compiler
                .compile_function(&function_definition.type_()),
        );

        let mut arguments = variables.clone();

        for (index, argument) in function_definition.arguments().iter().enumerate() {
            arguments.insert(
                argument.name().into(),
                llvm::get_param(entry_function, index as u32 + 1),
            );
        }

        let builder = llvm::Builder::new(entry_function);
        builder.position_at_end(builder.append_basic_block("entry"));
        builder.build_ret(
            ExpressionCompiler::new(&builder, &self, self.type_compiler)
                .compile(&function_definition.body(), &arguments)?,
        );

        llvm::verify_function(entry_function);

        Ok(entry_function)
    }

    fn generate_closure_entry_name(name: &str) -> String {
        [name, ".$entry"].concat()
    }
}
