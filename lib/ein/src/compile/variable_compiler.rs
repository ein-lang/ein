use super::type_compiler::TypeCompiler;
use crate::ast::*;
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
                .definitions()
                .iter()
                .filter_map(|definition| match definition {
                    Definition::FunctionDefinition(_) => None,
                    Definition::ValueDefinition(value_definition) => {
                        Some(value_definition.name().into())
                    }
                })
                .collect(),
        }
        .into()
    }

    pub fn compile(&self, variable: &Variable) -> ssf::ir::Expression {
        if self.names.contains(variable.name()) {
            ssf::ir::FunctionApplication::new(
                ssf::ir::Variable::new(variable.name()),
                vec![ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(self.type_compiler.compile_none(), 0),
                    vec![],
                )
                .into()],
            )
            .into()
        } else {
            ssf::ir::Variable::new(variable.name()).into()
        }
    }
}
