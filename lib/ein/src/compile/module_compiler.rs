use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct ModuleCompiler<'a> {
    module: &'a ast::Module,
    type_compiler: TypeCompiler,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(module: &'a ast::Module) -> Self {
        Self {
            module,
            type_compiler: TypeCompiler::new(module),
        }
    }

    pub fn compile(&self) -> Result<ssf::ast::Module, CompileError> {
        Ok(ssf::ast::Module::new(
            self.module
                .imported_modules()
                .iter()
                .flat_map(|module_interface| {
                    module_interface
                        .variables()
                        .iter()
                        .map(move |(name, type_)| {
                            ssf::ast::Declaration::new(
                                module_interface.path().fully_qualify_name(name),
                                self.type_compiler.compile(type_),
                            )
                        })
                })
                .collect(),
            self.module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    ast::Definition::FunctionDefinition(function_definition) => Ok(self
                        .compile_function_definition(function_definition)?
                        .into()),
                    ast::Definition::ValueDefinition(value_definition) => {
                        Ok(self.compile_value_definition(value_definition)?.into())
                    }
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
        ))
    }

    fn compile_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<ssf::ast::FunctionDefinition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_());

        Ok(ssf::ast::FunctionDefinition::new(
            function_definition.name(),
            vec![],
            function_definition
                .arguments()
                .iter()
                .zip(core_type.arguments())
                .map(|(name, type_)| ssf::ast::Argument::new(name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            ExpressionCompiler::new(&self.type_compiler).compile(
                function_definition.body(),
                &function_definition
                    .arguments()
                    .iter()
                    .zip(
                        // Reference types are resolved to bare function types
                        // by desugaring already here.
                        function_definition
                            .type_()
                            .to_function()
                            .expect("function type")
                            .arguments(),
                    )
                    .map(|(name, type_)| (name.clone(), type_.clone()))
                    .collect(),
            )?,
            core_type.result().clone(),
        ))
    }

    fn compile_value_definition(
        &self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<ssf::ast::ValueDefinition, CompileError> {
        Ok(ssf::ast::ValueDefinition::new(
            value_definition.name(),
            ExpressionCompiler::new(&self.type_compiler)
                .compile(value_definition.body(), &HashMap::new())?,
            self.type_compiler.compile_value(value_definition.type_()),
        ))
    }
}
