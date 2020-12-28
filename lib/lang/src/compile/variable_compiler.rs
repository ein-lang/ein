use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_compiler::TypeCompiler;
use crate::ast::*;
use std::collections::HashSet;
use std::sync::Arc;

pub struct VariableCompiler {
    type_compiler: Arc<TypeCompiler>,
    variable_names: HashSet<String>,
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
            variable_names: module
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
                        Some(name)
                    })
                })
                .collect::<Result<Vec<_>, CompileError>>()?
                .into_iter()
                .flatten()
                .map(String::from)
                .collect(),
        }
        .into())
    }

    pub fn compile(&self, variable: &Variable) -> ssf::ir::Expression {
        if self.variable_names.contains(variable.name()) {
            ssf::ir::FunctionApplication::new(
                ssf::ir::Variable::new(variable.name()),
                ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(self.type_compiler.compile_thunk_argument(), 0),
                    vec![],
                ),
            )
            .into()
        } else {
            ssf::ir::Variable::new(variable.name()).into()
        }
    }
}
