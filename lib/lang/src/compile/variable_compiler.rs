use super::{
    error::CompileError, reference_type_resolver::ReferenceTypeResolver,
    type_compiler::TypeCompiler,
};
use crate::{ast::*, types::Type};
use std::{collections::HashMap, sync::Arc};

pub struct VariableCompiler {
    type_compiler: Arc<TypeCompiler>,
    variables: HashMap<String, Type>,
}

impl VariableCompiler {
    pub fn new(
        type_compiler: Arc<TypeCompiler>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        module: &Module,
    ) -> Result<Arc<Self>, CompileError> {
        Ok(Self {
            type_compiler,
            // Assuming those names do not conflict with any local variables due to alpha conversion.
            variables: module
                .imports()
                .iter()
                .flat_map(|import| {
                    import
                        .module_interface()
                        .variables()
                        .iter()
                        .map(|(name, type_)| (name.as_str(), type_))
                })
                .chain(
                    module
                        .definitions()
                        .iter()
                        .map(|definition| (definition.name(), definition.type_())),
                )
                .map(|(name, type_)| {
                    Ok(if reference_type_resolver.is_function(type_)? {
                        None
                    } else {
                        Some((name.into(), type_.clone()))
                    })
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .flatten()
                .collect(),
        }
        .into())
    }

    pub fn compile(&self, variable: &Variable) -> Result<eir::ir::Expression, CompileError> {
        Ok(if let Some(type_) = self.variables.get(variable.name()) {
            eir::ir::FunctionApplication::new(
                eir::types::Function::new(
                    self.type_compiler.compile_thunk_argument(),
                    self.type_compiler.compile(type_)?,
                ),
                eir::ir::Variable::new(variable.name()),
                eir::ir::Record::new(self.type_compiler.compile_thunk_argument(), vec![]),
            )
            .into()
        } else {
            eir::ir::Variable::new(variable.name()).into()
        })
    }
}
