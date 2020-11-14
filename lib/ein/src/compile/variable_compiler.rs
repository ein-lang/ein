use super::type_compiler::TypeCompiler;
use crate::ast::*;
use crate::types::Type;
use std::collections::HashSet;
use std::sync::Arc;

pub struct VariableCompiler {
    type_compiler: Arc<TypeCompiler>,
    names: HashSet<String>,
}

impl VariableCompiler {
    pub fn new(type_compiler: Arc<TypeCompiler>, module: &Module) -> Arc<Self> {
        Self {
            type_compiler,
            // Assuming those names do not conflict with any local variables due to alpha conversion.
            names: module
                .imports()
                .iter()
                .flat_map(|import| {
                    import
                        .module_interface()
                        .variables()
                        .iter()
                        .filter_map(|(name, type_)| match type_ {
                            Type::Function(_) => None,
                            _ => Some(name.into()),
                        })
                })
                .chain(
                    module
                        .definitions()
                        .iter()
                        .filter_map(|definition| match definition {
                            Definition::FunctionDefinition(_) => None,
                            Definition::ValueDefinition(value_definition) => {
                                Some(value_definition.name().into())
                            }
                        }),
                )
                .collect(),
        }
        .into()
    }

    pub fn compile(&self, variable: &Variable) -> ssf::ir::Expression {
        if self.names.contains(variable.name()) {
            ssf::ir::FunctionApplication::new(
                ssf::ir::Variable::new(variable.name()),
                ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(self.type_compiler.compile_none(), 0),
                    vec![],
                ),
            )
            .into()
        } else {
            ssf::ir::Variable::new(variable.name()).into()
        }
    }
}
