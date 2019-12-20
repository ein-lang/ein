use super::error::CompileError;
use crate::ast::Module;
use crate::ast::ModuleInterface;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ModuleInterfaceCompiler {}

impl ModuleInterfaceCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, module: &Module) -> Result<ModuleInterface, CompileError> {
        let definitions = module
            .definitions()
            .iter()
            .map(|definition| (definition.name(), definition.type_()))
            .collect::<HashMap<&str, &Type>>();

        Ok(ModuleInterface::new(
            module.path().clone(),
            module
                .export()
                .names()
                .iter()
                .map(|name| {
                    let type_ = *definitions
                        .get(name.as_str())
                        .ok_or_else(|| CompileError::ExportedNameNotFound { name: name.into() })?;

                    Ok((name.into(), type_.clone()))
                })
                .collect::<Result<HashMap<_, _>, CompileError>>()?,
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
