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
                .foreign_declarations()
                .iter()
                .map(|declaration| -> Result<_, CompileError> {
                    Ok(ssf::ir::ForeignDeclaration::new(
                        declaration.name(),
                        declaration.foreign_name(),
                        self.type_compiler
                            .compile(declaration.type_())?
                            .into_function()
                            .ok_or_else(|| {
                                CompileError::FunctionExpected(
                                    declaration.source_information().clone(),
                                )
                            })?,
                    ))
                })
                .collect::<Result<_, _>>()?,
            module
                .imports()
                .iter()
                .flat_map(|import| {
                    import
                        .module_interface()
                        .variables()
                        .iter()
                        .map(|(name, type_)| {
                            let type_ = self.type_compiler.compile(type_)?;

                            Ok(ssf::ir::Declaration::new(
                                name,
                                if let ssf::types::Type::Function(function_type) = type_ {
                                    function_type
                                } else {
                                    ssf::types::Function::new(
                                        self.type_compiler.compile_thunk_argument(),
                                        type_,
                                    )
                                },
                            ))
                        })
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .chain(vec![ssf::ir::Declaration::new(
                    &self.string_type_configuration.equal_function_name,
                    ssf::types::Function::new(
                        self.type_compiler.compile_string(),
                        ssf::types::Function::new(
                            self.type_compiler.compile_string(),
                            self.type_compiler.compile_boolean(),
                        ),
                    ),
                )])
                .collect(),
            module
                .definitions()
                .iter()
                .map(|definition| {
                    Ok(match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            vec![self.compile_function_definition(function_definition)?]
                        }
                        Definition::VariableDefinition(variable_definition) => {
                            self.compile_variable_definition(variable_definition)?
                        }
                    })
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .flatten()
                .collect(),
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
    ) -> Result<Vec<ssf::ir::Definition>, CompileError> {
        let variable_type = self.type_compiler.compile(variable_definition.type_())?;
        let thunk_argument_type = self.type_compiler.compile_thunk_argument();
        const THUNK_ARGUMENT_NAME: &str = "$thunk_arg";

        Ok(
            if let ssf::types::Type::Function(function_type) = &variable_type {
                let thunk_name = format!("{}.thunk", variable_definition.name());
                const ARGUMENT_NAME: &str = "$arg";

                vec![
                    ssf::ir::Definition::thunk(
                        &thunk_name,
                        vec![ssf::ir::Argument::new(
                            THUNK_ARGUMENT_NAME,
                            thunk_argument_type.clone(),
                        )],
                        self.expression_compiler
                            .compile(variable_definition.body())?,
                        variable_type.clone(),
                    ),
                    ssf::ir::Definition::new(
                        variable_definition.name(),
                        vec![ssf::ir::Argument::new(
                            ARGUMENT_NAME,
                            function_type.argument().clone(),
                        )],
                        ssf::ir::FunctionApplication::new(
                            ssf::ir::FunctionApplication::new(
                                ssf::ir::Variable::new(&thunk_name),
                                ssf::ir::ConstructorApplication::new(
                                    ssf::ir::Constructor::new(thunk_argument_type, 0),
                                    vec![],
                                ),
                            ),
                            ssf::ir::Variable::new(ARGUMENT_NAME),
                        ),
                        function_type.result().clone(),
                    ),
                ]
            } else {
                vec![ssf::ir::Definition::thunk(
                    variable_definition.name(),
                    vec![ssf::ir::Argument::new(
                        THUNK_ARGUMENT_NAME,
                        thunk_argument_type,
                    )],
                    self.expression_compiler
                        .compile(variable_definition.body())?,
                    variable_type.clone(),
                )]
            },
        )
    }
}
