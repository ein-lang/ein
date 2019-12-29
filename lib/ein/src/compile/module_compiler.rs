use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ModuleCompiler<'a> {
    module: &'a ast::Module,
    reference_type_resolver: Rc<ReferenceTypeResolver>,
    type_compiler: TypeCompiler,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(module: &'a ast::Module) -> Self {
        let reference_type_resolver = Rc::new(ReferenceTypeResolver::new(module));

        Self {
            module,
            reference_type_resolver: reference_type_resolver.clone(),
            type_compiler: TypeCompiler::new(reference_type_resolver),
        }
    }

    pub fn compile(&self) -> Result<core::ast::Module, CompileError> {
        Ok(core::ast::Module::new(
            self.module
                .imported_modules()
                .iter()
                .flat_map(|module_interface| {
                    module_interface
                        .variables()
                        .iter()
                        .map(move |(name, type_)| {
                            core::ast::Declaration::new(
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
    ) -> Result<core::ast::FunctionDefinition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_());

        Ok(core::ast::FunctionDefinition::new(
            function_definition.name(),
            vec![],
            function_definition
                .arguments()
                .iter()
                .zip(core_type.arguments())
                .map(|(name, type_)| core::ast::Argument::new(name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            ExpressionCompiler::new(&self.type_compiler).compile(
                function_definition.body(),
                &function_definition
                    .arguments()
                    .iter()
                    .zip(
                        self.reference_type_resolver
                            .resolve(function_definition.type_())
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
    ) -> Result<core::ast::ValueDefinition, CompileError> {
        Ok(core::ast::ValueDefinition::new(
            value_definition.name(),
            ExpressionCompiler::new(&self.type_compiler)
                .compile(value_definition.body(), &HashMap::new())?,
            self.type_compiler.compile_value(value_definition.type_()),
        ))
    }
}
