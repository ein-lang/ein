use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast::*;
use std::sync::Arc;

pub struct ModuleCompiler {
    expression_compiler: Arc<ExpressionCompiler>,
    type_compiler: Arc<TypeCompiler>,
}

impl ModuleCompiler {
    pub fn new(
        expression_compiler: Arc<ExpressionCompiler>,
        type_compiler: Arc<TypeCompiler>,
    ) -> Self {
        Self {
            expression_compiler,
            type_compiler,
        }
    }

    pub fn compile(&self, module: &Module) -> Result<ssf::ir::Module, CompileError> {
        Ok(ssf::ir::Module::new(
            module
                .imports()
                .iter()
                .flat_map(|import| {
                    import
                        .module_interface()
                        .variables()
                        .iter()
                        .map(move |(name, type_)| {
                            Ok(ssf::ir::Declaration::new(name, {
                                match self.type_compiler.compile(type_)? {
                                    ssf::types::Type::Function(function_type) => function_type,
                                    ssf::types::Type::Value(value_type) => {
                                        ssf::types::Function::new(
                                            vec![self.type_compiler.compile_none().into()],
                                            value_type,
                                        )
                                    }
                                }
                            }))
                        })
                })
                .collect::<Result<_, CompileError>>()?,
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    Definition::FunctionDefinition(function_definition) => Ok(self
                        .compile_function_definition(function_definition)?),
                    Definition::ValueDefinition(value_definition) => {
                        Ok(self.compile_value_definition(value_definition)?)
                    }
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
        )?)
    }

    fn compile_function_definition(
        &self,
        function_definition: &FunctionDefinition,
    ) -> Result<ssf::ir::Definition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_())?;

        Ok(ssf::ir::Definition::new(
            function_definition.name(),
            function_definition
                .arguments()
                .iter()
                .zip(core_type.arguments())
                .map(|(name, type_)| ssf::ir::Argument::new(name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            self.expression_compiler
                .compile(function_definition.body())?,
            core_type.result().clone(),
        ))
    }

    fn compile_value_definition(
        &self,
        value_definition: &ValueDefinition,
    ) -> Result<ssf::ir::Definition, CompileError> {
        Ok(ssf::ir::Definition::thunk(
            value_definition.name(),
            vec![ssf::ir::Argument::new(
                "$thunk_arg",
                self.type_compiler.compile_none(),
            )],
            self.expression_compiler.compile(value_definition.body())?,
            self.type_compiler.compile_value(value_definition.type_())?,
        ))
    }
}
