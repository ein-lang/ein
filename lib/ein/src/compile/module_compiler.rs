use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast;

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

    pub fn compile(&self) -> Result<ssf::ir::Module, CompileError> {
        Ok(ssf::ir::Module::new(
            self.module
                .imported_modules()
                .iter()
                .flat_map(|module_interface| {
                    module_interface
                        .variables()
                        .iter()
                        .map(move |(name, type_)| {
                            ssf::ir::Declaration::new(
                                module_interface.path().qualify_name(name),
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
        )?)
    }

    fn compile_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<ssf::ir::FunctionDefinition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_());

        Ok(ssf::ir::FunctionDefinition::new(
            function_definition.name(),
            function_definition
                .arguments()
                .iter()
                .zip(core_type.arguments())
                .map(|(name, type_)| ssf::ir::Argument::new(name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            ExpressionCompiler::new(&self.type_compiler).compile(function_definition.body())?,
            core_type.result().clone(),
        ))
    }

    fn compile_value_definition(
        &self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<ssf::ir::ValueDefinition, CompileError> {
        Ok(ssf::ir::ValueDefinition::new(
            value_definition.name(),
            ExpressionCompiler::new(&self.type_compiler).compile(value_definition.body())?,
            self.type_compiler.compile_value(value_definition.type_()),
        ))
    }
}
