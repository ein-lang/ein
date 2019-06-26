use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct FunctionCompiler<'a> {
    module: &'a llvm::Module,
    type_compiler: &'a TypeCompiler,
    global_variables: &'a HashMap<String, llvm::Value>,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(
        module: &'a llvm::Module,
        type_compiler: &'a TypeCompiler,
        global_variables: &'a HashMap<String, llvm::Value>,
    ) -> Self {
        Self {
            module,
            type_compiler,
            global_variables,
        }
    }

    pub unsafe fn compile(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<llvm::Value, CompileError> {
        let entry_function = self.module.add_function(
            &Self::generate_closure_entry_name(function_definition.name()),
            self.type_compiler
                .compile_function(&function_definition.type_()),
        );

        let builder = llvm::Builder::new(entry_function);
        builder.position_at_end(builder.append_basic_block("entry"));

        let environment = builder.build_bit_cast(
            llvm::get_param(entry_function, 0),
            llvm::Type::pointer(
                self.type_compiler
                    .compile_environment(function_definition.environment()),
            ),
        );

        let mut variables = self.global_variables.clone();

        for (index, free_variable) in function_definition.environment().iter().enumerate() {
            variables.insert(
                free_variable.name().into(),
                builder.build_load(builder.build_gep(
                    environment,
                    &[
                        llvm::const_int(llvm::Type::i32(), 0),
                        llvm::const_int(llvm::Type::i32(), index as u64),
                    ],
                )),
            );
        }

        for (index, argument) in function_definition.arguments().iter().enumerate() {
            variables.insert(
                argument.name().into(),
                llvm::get_param(entry_function, index as u32 + 1),
            );
        }

        builder.build_ret(
            ExpressionCompiler::new(&builder, &self, self.type_compiler)
                .compile(&function_definition.body(), &variables)?,
        );

        llvm::verify_function(entry_function);

        Ok(entry_function)
    }

    fn generate_closure_entry_name(name: &str) -> String {
        [name, ".$entry"].concat()
    }
}
