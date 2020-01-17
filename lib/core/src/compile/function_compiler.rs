use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct FunctionCompiler<'a> {
    context: &'a llvm::Context,
    module: &'a llvm::Module,
    type_compiler: &'a TypeCompiler<'a>,
    global_variables: &'a HashMap<String, llvm::Value>,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(
        context: &'a llvm::Context,
        module: &'a llvm::Module,
        type_compiler: &'a TypeCompiler<'a>,
        global_variables: &'a HashMap<String, llvm::Value>,
    ) -> Self {
        Self {
            context,
            module,
            type_compiler,
            global_variables,
        }
    }

    pub fn compile(
        &mut self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<llvm::Value, CompileError> {
        let closure_type = self.type_compiler.compile_closure(function_definition);

        let entry_function = self.module.add_function(
            &Self::generate_closure_entry_name(function_definition.name()),
            closure_type.struct_elements()[0].element(),
        );

        let builder = llvm::Builder::new(entry_function);
        builder.position_at_end(builder.append_basic_block("entry"));

        let environment = builder.build_bit_cast(
            entry_function.get_param(0),
            self.context.pointer_type(closure_type.struct_elements()[1]),
        );

        let mut variables = self.global_variables.clone();

        for (index, free_variable) in function_definition.environment().iter().enumerate() {
            variables.insert(
                free_variable.name().into(),
                builder.build_load(builder.build_gep(
                    environment,
                    &[
                        llvm::const_int(self.context.i32_type(), 0),
                        llvm::const_int(self.context.i32_type(), index as u64),
                    ],
                )),
            );
        }

        for (index, argument) in function_definition.arguments().iter().enumerate() {
            variables.insert(
                argument.name().into(),
                entry_function.get_param(index as u32 + 1),
            );
        }

        builder.build_ret(
            ExpressionCompiler::new(self.context, &builder, self, self.type_compiler)
                .compile(&function_definition.body(), &variables)?,
        );

        entry_function.verify_function();

        Ok(entry_function)
    }

    fn generate_closure_entry_name(name: &str) -> String {
        [name, ".$entry"].concat()
    }
}
