use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ModuleCompiler {
    expression_compiler: Arc<ExpressionCompiler>,
    type_compiler: Arc<TypeCompiler>,
    global_names: Arc<HashMap<String, String>>,
}

impl ModuleCompiler {
    pub fn new(
        expression_compiler: Arc<ExpressionCompiler>,
        type_compiler: Arc<TypeCompiler>,
        global_names: Arc<HashMap<String, String>>,
    ) -> Self {
        Self {
            expression_compiler,
            type_compiler,
            global_names,
        }
    }

    pub fn compile(&self, module: &Module) -> Result<eir::ir::Module, CompileError> {
        Ok(eir::ir::Module::new(
            todo!(),
            module
                .import_foreigns()
                .iter()
                .map(|import| -> Result<_, CompileError> {
                    Ok(eir::ir::ForeignDeclaration::new(
                        import.name(),
                        import.foreign_name(),
                        self.type_compiler
                            .compile(import.type_())?
                            .into_function()
                            .ok_or_else(|| {
                                CompileError::FunctionExpected(import.source_information().clone())
                            })?,
                        match import.calling_convention() {
                            CallingConvention::Native => eir::ir::CallingConvention::Source,
                            CallingConvention::C => eir::ir::CallingConvention::Target,
                        },
                    ))
                })
                .collect::<Result<_, _>>()?,
            module
                .export_foreign()
                .names()
                .iter()
                .map(|name| {
                    Ok(eir::ir::ForeignDefinition::new(
                        self.global_names.get(name).ok_or_else(|| {
                            CompileError::ExportedNameNotFound { name: name.clone() }
                        })?,
                        name,
                    ))
                })
                .collect::<Result<_, CompileError>>()?,
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

                            Ok(eir::ir::Declaration::new(
                                name,
                                if let eir::types::Type::Function(function_type) = type_ {
                                    function_type
                                } else {
                                    eir::types::Function::new(
                                        self.type_compiler.compile_thunk_argument(),
                                        type_,
                                    )
                                },
                            ))
                        })
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
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
        ))
    }

    fn compile_function_definition(
        &self,
        function_definition: &FunctionDefinition,
    ) -> Result<eir::ir::Definition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_())?;

        Ok(eir::ir::Definition::new(
            function_definition.name(),
            function_definition
                .arguments()
                .iter()
                .zip(core_type.arguments())
                .map(|(name, type_)| eir::ir::Argument::new(name.clone(), type_.clone()))
                .collect::<Vec<_>>(),
            self.expression_compiler
                .compile(function_definition.body())?,
            (0..function_definition.arguments().len())
                .fold(core_type.into(), |type_: eir::types::Type, _| {
                    type_.into_function().unwrap().result().clone()
                }),
        ))
    }

    fn compile_variable_definition(
        &self,
        variable_definition: &VariableDefinition,
    ) -> Result<Vec<eir::ir::Definition>, CompileError> {
        let core_type = self.type_compiler.compile(variable_definition.type_())?;

        Ok(
            if let eir::types::Type::Function(function_type) = core_type {
                self.compile_function_variable_definition(
                    variable_definition.name(),
                    variable_definition.body(),
                    &function_type,
                )?
            } else {
                vec![self.compile_value_variable_definition(
                    variable_definition.name(),
                    variable_definition.body(),
                    &core_type,
                )?]
            },
        )
    }

    fn compile_function_variable_definition(
        &self,
        name: &str,
        body: &Expression,
        function_type: &eir::types::Function,
    ) -> Result<Vec<eir::ir::Definition>, CompileError> {
        let thunk_name = format!("{}.thunk", name);
        const ARGUMENT_NAME: &str = "$arg";

        Ok(vec![
            eir::ir::Definition::thunk(
                &thunk_name,
                vec![eir::ir::Argument::new(
                    "",
                    self.type_compiler.compile_thunk_argument(),
                )],
                self.expression_compiler.compile(body)?,
                function_type.clone(),
            ),
            eir::ir::Definition::new(
                name,
                vec![eir::ir::Argument::new(
                    ARGUMENT_NAME,
                    function_type.argument().clone(),
                )],
                eir::ir::FunctionApplication::new(
                    eir::ir::FunctionApplication::new(
                        eir::ir::Variable::new(&thunk_name),
                        eir::ir::Record::new(self.type_compiler.compile_thunk_argument(), vec![]),
                    ),
                    eir::ir::Variable::new(ARGUMENT_NAME),
                ),
                function_type.result().clone(),
            ),
        ])
    }

    fn compile_value_variable_definition(
        &self,
        name: &str,
        body: &Expression,
        type_: &eir::types::Type,
    ) -> Result<eir::ir::Definition, CompileError> {
        Ok(eir::ir::Definition::thunk(
            name,
            vec![eir::ir::Argument::new(
                "",
                self.type_compiler.compile_thunk_argument(),
            )],
            self.expression_compiler.compile(body)?,
            type_.clone(),
        ))
    }
}
