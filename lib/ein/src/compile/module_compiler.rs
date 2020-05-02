use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::type_compiler::TypeCompiler;
use crate::ast::*;
use crate::types::Type;
use std::rc::Rc;

pub struct ModuleCompiler {
    expression_compiler: Rc<ExpressionCompiler>,
    type_compiler: Rc<TypeCompiler>,
}

impl ModuleCompiler {
    pub fn new(
        expression_compiler: Rc<ExpressionCompiler>,
        type_compiler: Rc<TypeCompiler>,
    ) -> Self {
        Self {
            expression_compiler,
            type_compiler,
        }
    }

    pub fn compile(&self, module: &Module) -> Result<ssf::ir::Module, CompileError> {
        Ok(ssf::ir::Module::new(
            module
                .imported_modules()
                .iter()
                .flat_map(|module_interface| {
                    module_interface
                        .variables()
                        .iter()
                        .map(move |(name, type_)| {
                            Ok(ssf::ir::Declaration::new(
                                module_interface.path().qualify_name(name),
                                self.type_compiler.compile(type_)?,
                            ))
                        })
                })
                .collect::<Result<_, CompileError>>()?,
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    Definition::FunctionDefinition(function_definition) => Ok(self
                        .compile_function_definition(function_definition)?
                        .into()),
                    Definition::ValueDefinition(value_definition) => {
                        Ok(self.compile_value_definition(value_definition)?.into())
                    }
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .chain(
                    module
                        .type_definitions()
                        .iter()
                        .map(|type_definition| self.compile_type_definition(type_definition))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .flatten(),
                )
                .collect(),
        )?)
    }

    fn compile_type_definition(
        &self,
        type_definition: &TypeDefinition,
    ) -> Result<Vec<ssf::ir::Definition>, CompileError> {
        if let Type::Record(record_type) = type_definition.type_() {
            let algebraic_type = self.type_compiler.compile_record(record_type)?;

            record_type
                .elements()
                .iter()
                .map(|(key, type_)| {
                    Ok(ssf::ir::FunctionDefinition::new(
                        format!("{}.{}", type_definition.name(), key),
                        vec![ssf::ir::Argument::new("x", algebraic_type.clone())],
                        ssf::ir::AlgebraicCase::new(
                            ssf::ir::Variable::new("x"),
                            vec![ssf::ir::AlgebraicAlternative::new(
                                ssf::ir::Constructor::new(algebraic_type.clone(), 0),
                                record_type
                                    .elements()
                                    .keys()
                                    .map(|key| format!("${}", key))
                                    .collect(),
                                ssf::ir::Variable::new(format!("${}", key)),
                            )],
                            None,
                        ),
                        self.type_compiler.compile_value(type_)?,
                    )
                    .into())
                })
                .collect()
        } else {
            Ok(vec![])
        }
    }

    fn compile_function_definition(
        &self,
        function_definition: &FunctionDefinition,
    ) -> Result<ssf::ir::FunctionDefinition, CompileError> {
        let core_type = self
            .type_compiler
            .compile_function(function_definition.type_())?;

        Ok(ssf::ir::FunctionDefinition::new(
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
    ) -> Result<ssf::ir::ValueDefinition, CompileError> {
        Ok(ssf::ir::ValueDefinition::new(
            value_definition.name(),
            self.expression_compiler.compile(value_definition.body())?,
            self.type_compiler.compile_value(value_definition.type_())?,
        ))
    }
}
