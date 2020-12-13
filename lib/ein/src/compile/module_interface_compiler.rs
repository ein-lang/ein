use super::error::CompileError;
use crate::ast::*;

#[derive(Debug)]
pub struct ModuleInterfaceCompiler {}

impl ModuleInterfaceCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, module: &Module) -> Result<ModuleInterface, CompileError> {
        if let Some(name) = module.export().names().iter().find(|name| {
            let exported_name = module.path().fully_qualify_name(name);

            !module
                .type_definitions()
                .iter()
                .map(TypeDefinition::name)
                .chain(module.definitions().iter().map(Definition::name))
                .any(|name| name == exported_name)
        }) {
            Err(CompileError::ExportedNameNotFound { name: name.into() })
        } else {
            Ok(ModuleInterface::new(
                module.path().clone(),
                module.export().names().iter().cloned().collect(),
                module
                    .type_definitions()
                    .iter()
                    .map(|type_definition| {
                        (
                            type_definition.name().into(),
                            type_definition.type_().clone(),
                        )
                    })
                    .collect(),
                module
                    .definitions()
                    .iter()
                    .filter_map(|definition| match definition {
                        Definition::FunctionDefinition(function_definition) => Some((
                            function_definition.name().into(),
                            function_definition.type_().clone(),
                        )),
                        Definition::VariableDefinition(_) => None,
                    })
                    .collect(),
                module
                    .definitions()
                    .iter()
                    .filter_map(|definition| match definition {
                        Definition::FunctionDefinition(_) => None,
                        Definition::VariableDefinition(variable_definition) => Some((
                            variable_definition.name().into(),
                            variable_definition.type_().clone(),
                        )),
                    })
                    .collect(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Export, Number, VariableDefinition};
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn compile_empty_module_interface() {
        assert_eq!(
            ModuleInterfaceCompiler::new().compile(&Module::from_definitions(vec![]),),
            Ok(ModuleInterface::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ))
        );
    }

    #[test]
    fn compile_module_interface_with_definition() {
        assert_eq!(
            ModuleInterfaceCompiler::new().compile(&Module::new(
                ModulePath::new(Package::new("P", ""), vec!["M".into()]),
                Export::new(vec!["x".into()].into_iter().collect()),
                vec![],
                BuiltinInterface::dummy(),
                vec![],
                vec![VariableDefinition::new(
                    "P().M.x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            ),),
            Ok(ModuleInterface::new(
                ModulePath::new(Package::new("P", ""), vec!["M".into()]),
                vec!["x".into()].into_iter().collect(),
                Default::default(),
                Default::default(),
                vec![(
                    "P().M.x".into(),
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
                ModulePath::new(Package::new("P", ""), vec!["M".into()]),
                Export::new(vec!["x".into()].into_iter().collect()),
                vec![],
                BuiltinInterface::dummy(),
                vec![],
                vec![],
            ),),
            Err(CompileError::ExportedNameNotFound { name: "x".into() })
        );
    }
}
