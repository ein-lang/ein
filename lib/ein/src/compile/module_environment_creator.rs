use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;

pub struct ModuleEnvironmentCreator {}

impl ModuleEnvironmentCreator {
    pub fn create(module: &Module) -> HashMap<String, Type> {
        let mut variables = HashMap::<String, Type>::new();

        for import in module.imports() {
            for (name, type_) in import.module_interface().functions() {
                variables.insert(name.into(), type_.clone());
            }

            for (name, type_) in import.module_interface().variables() {
                variables.insert(name.into(), type_.clone());
            }
        }

        for interface in module.ffi_imports() {
            for (name, type_) in interface.functions() {
                variables.insert(name.into(), type_.clone().into());
            }
        }

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    variables.insert(
                        function_definition.name().into(),
                        function_definition.type_().clone(),
                    );
                }
                Definition::VariableDefinition(variable_definition) => {
                    variables.insert(
                        variable_definition.name().into(),
                        variable_definition.type_().clone(),
                    );
                }
            }
        }

        variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::package::*;
    use crate::path::*;
    use crate::types;

    #[test]
    fn include_ffi_functions() {
        assert_eq!(
            ModuleEnvironmentCreator::create(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![BuiltinInterface::new(
                    Default::default(),
                    vec![(
                        "foo".into(),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                    )]
                    .into_iter()
                    .collect(),
                )],
                vec![],
                vec![],
            )),
            vec![(
                "foo".into(),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            )]
            .into_iter()
            .collect()
        );
    }
}
