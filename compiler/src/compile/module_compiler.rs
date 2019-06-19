use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;

pub struct ModuleCompiler {
    expression_compiler: ExpressionCompiler,
    type_compiler: TypeCompiler,
}

impl ModuleCompiler {
    pub fn new() -> Self {
        Self {
            expression_compiler: ExpressionCompiler::new(),
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
        Ok(core::ast::FunctionDefinition::new(
            function_definition.name().into(),
            vec![],
            function_definition
                .arguments()
                .iter()
                .zip(function_definition.type_().arguments())
                .map(|(name, type_)| {
                    core::ast::Argument::new(name.clone(), self.type_compiler.compile(type_))
                })
                .collect::<Vec<_>>(),
            self.expression_compiler
                .compile(function_definition.body())?,
            self.type_compiler
                .compile_value(function_definition.type_().last_result()),
        ))
    }

    fn compile_value_definition(
        &self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<core::ast::ValueDefinition, CompileError> {
        Ok(core::ast::ValueDefinition::new(
            value_definition.name().into(),
            self.expression_compiler.compile(value_definition.body())?,
            self.type_compiler.compile_value(value_definition.type_()),
        ))
    }
}
