use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct ModuleCompiler {
    type_compiler: TypeCompiler,
}

impl ModuleCompiler {
    pub fn new() -> Self {
        Self {
            type_compiler: TypeCompiler::new(),
        }
    }

    pub fn compile(&self, module: &ast::Module) -> Result<core::ast::Module, CompileError> {
        Ok(core::ast::Module::new(
            module
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
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn compile_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<core::ast::FunctionDefinition, CompileError> {
        let type_ = function_definition
            .type_()
            .to_function()
            .ok_or(CompileError::new("function expected"))?;

        Ok(core::ast::FunctionDefinition::new(
            function_definition.name().into(),
            vec![],
            function_definition
                .arguments()
                .iter()
                .zip(type_.arguments())
                .map(|(name, type_)| {
                    core::ast::Argument::new(name.clone(), self.type_compiler.compile(type_))
                })
                .collect::<Vec<_>>(),
            ExpressionCompiler::new(&self.type_compiler).compile(
                function_definition.body(),
                &function_definition
                    .arguments()
                    .iter()
                    .zip(type_.arguments())
                    .map(|(name, type_)| (name.clone(), type_.clone()))
                    .collect(),
            )?,
            self.type_compiler.compile_value(type_.last_result()),
        ))
    }

    fn compile_value_definition(
        &self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<core::ast::ValueDefinition, CompileError> {
        Ok(core::ast::ValueDefinition::new(
            value_definition.name().into(),
            ExpressionCompiler::new(&self.type_compiler)
                .compile(value_definition.body(), &HashMap::new())?,
            self.type_compiler.compile_value(value_definition.type_()),
        ))
    }
}
