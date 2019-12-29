use super::error::CompileError;
use crate::ast::Module;
use crate::ast::ModuleInterface;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ModuleInterfaceCompiler {}

impl ModuleInterfaceCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, module: &Module) -> Result<ModuleInterface, CompileError> {
        for name in module.export().names() {
            if !module
                .type_definitions()
                .iter()
                .any(|definition| definition.name() == name)
                && !module
                    .definitions()
                    .iter()
                    .any(|definition| definition.name() == name)
            {
                return Err(CompileError::ExportedNameNotFound { name: name.into() });
            }
        }

        Ok(ModuleInterface::new(
            module.path().clone(),
            module
                .type_definitions()
                .iter()
                .map(|type_definition| {
                    (
                        type_definition.name().into(),
                        type_definition.type_().clone(),
                    )
                })
                .collect::<HashMap<_, _>>(),
            module
                .definitions()
                .iter()
                .map(|definition| (definition.name().into(), definition.type_().clone()))
                .collect::<HashMap<_, _>>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Export, Number, ValueDefinition};
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use crate::types;

    #[test]
    fn compile_empty_module_interface() {
        assert_eq!(
            ModuleInterfaceCompiler::new().compile(&Module::from_definitions(vec![])),
            Ok(ModuleInterface::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Default::default(),
                Default::default()
            ))
        );
    }

    #[test]
    fn compile_module_interface_with_definition() {
        assert_eq!(
            ModuleInterfaceCompiler::new().compile(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(vec!["x".into()].into_iter().collect()),
                vec![],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            )),
            Ok(ModuleInterface::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Default::default(),
                vec![(
                    "x".into(),
                    types::Number::new(SourceInformation::dummy()).into()
                )]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn fail_to_compile_module_interface_due_to_missing_exported_name() {
        assert_eq!(
            ModuleInterfaceCompiler::new().compile(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(vec!["x".into()].into_iter().collect()),
                vec![],
                vec![],
                vec![],
            )),
            Err(CompileError::ExportedNameNotFound { name: "x".into() })
        );
    }
}
