use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::string_type_configuration::StringTypeConfiguration;
use super::type_compiler::TypeCompiler;
use crate::ast::*;
use std::sync::Arc;

pub struct ModuleCompiler {
    expression_compiler: Arc<ExpressionCompiler>,
    type_compiler: Arc<TypeCompiler>,
    string_type_configuration: Arc<StringTypeConfiguration>,
}

impl ModuleCompiler {
    pub fn new(
        expression_compiler: Arc<ExpressionCompiler>,
        type_compiler: Arc<TypeCompiler>,
        string_type_configuration: Arc<StringTypeConfiguration>,
    ) -> Self {
        Self {
            expression_compiler,
            type_compiler,
            string_type_configuration,
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
                        .functions()
                        .iter()
                        .map(|(name, type_)| {
                            Ok(ssf::ir::Declaration::new(
                                name,
                                self.type_compiler.compile(type_)?.into_function().unwrap(),
                            ))
                        })
                        .chain(
                            import
                                .module_interface()
                                .variables()
                                .iter()
                                .map(|(name, type_)| {
                                    Ok(ssf::ir::Declaration::new(
                                        name,
                                        ssf::types::Function::new(
                                            self.type_compiler.compile_none(),
                                            self.type_compiler.compile(type_)?,
                                        ),
                                    ))
                                }),
                        )
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .chain(vec![ssf::ir::Declaration::new(
                    &self.string_type_configuration.equal_function_name,
                    ssf::types::Function::new(
                        ssf::types::Primitive::Integer64,
                        ssf::types::Function::new(
                            ssf::types::Primitive::Integer64,
                            self.type_compiler.compile_boolean(),
                        ),
                    ),
                )])
                .collect(),
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    Definition::FunctionDefinition(function_definition) => {
                        Ok(self.compile_function_definition(function_definition)?)
                    }
                    Definition::VariableDefinition(variable_definition) => {
                        Ok(self.compile_variable_definition(variable_definition)?)
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
            (0..function_definition.arguments().len())
                .fold(core_type.into(), |type_: ssf::types::Type, _| {
                    type_.into_function().unwrap().result().clone()
                }),
        ))
    }

    fn compile_variable_definition(
        &self,
        variable_definition: &VariableDefinition,
    ) -> Result<ssf::ir::Definition, CompileError> {
        Ok(ssf::ir::Definition::thunk(
            variable_definition.name(),
            vec![ssf::ir::Argument::new(
                "$thunk_arg",
                self.type_compiler.compile_none(),
            )],
            self.expression_compiler
                .compile(variable_definition.body())?,
            self.type_compiler.compile(variable_definition.type_())?,
        ))
    }
}
