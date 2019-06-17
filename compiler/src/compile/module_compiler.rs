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
                    ast::Definition::FunctionDefinition(function_definition) => {
                        self.compile_function_definition(function_definition).into()
                    }
                    ast::Definition::VariableDefinition(variable_definition) => {
                        self.compile_variable_definition(variable_definition).into()
                    }
                })
                .collect::<Vec<_>>(),
        ))
    }

    fn compile_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
    ) -> core::ast::FunctionDefinition {
        core::ast::FunctionDefinition::new(
            function_definition.name().into(),
            function_definition
                .arguments()
                .iter()
                .zip(function_definition.type_().arguments())
                .map(|(name, type_)| {
                    core::ast::Argument::new(name.clone(), self.type_compiler.compile(type_))
                })
                .collect::<Vec<_>>(),
            self.expression_compiler.compile(function_definition.body()),
            self.type_compiler
                .compile_value(function_definition.type_().last_result()),
        )
    }

    fn compile_variable_definition(
        &self,
        variable_definition: &ast::VariableDefinition,
    ) -> core::ast::VariableDefinition {
        core::ast::VariableDefinition::new(
            variable_definition.name().into(),
            self.expression_compiler.compile(variable_definition.body()),
            self.type_compiler
                .compile_value(variable_definition.type_()),
        )
    }
}
