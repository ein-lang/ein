use crate::ast::*;
use crate::types::*;
use std::collections::HashMap;

pub fn remove_reference_types(module: &Module) -> Module {
    let mut environment = HashMap::<String, Type>::new();

    for imported_module in module.imported_modules() {
        environment.extend(
            imported_module
                .types()
                .iter()
                .map(|(name, type_)| (imported_module.path().qualify_name(name), type_.clone())),
        );
    }

    environment.extend(module.type_definitions().iter().map(|type_definition| {
        (
            type_definition.name().into(),
            type_definition.type_().clone(),
        )
    }));

    let mut type_definitions = vec![];

    for type_definition in module.type_definitions() {
        type_definitions.push(type_definition.resolve_reference_types(&environment));
    }

    Module::new(
        module.path().clone(),
        module.export().clone(),
        module.imported_modules().to_vec(),
        type_definitions,
        module
            .definitions()
            .iter()
            .map(|definition| definition.resolve_reference_types(&environment))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::package::*;
    use crate::path::*;
    use crate::types;

    #[test]
    fn do_not_remove_reference_types_if_none_exists() {
        assert_eq!(remove_reference_types(&Module::dummy()), Module::dummy());
    }

    mod internal_types {
        use super::*;

        #[test]
        fn remove_reference_type_in_type_definition() {
            assert_eq!(
                remove_reference_types(&Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![],
                    vec![
                        TypeDefinition::new("Foo", types::Number::new(SourceInformation::dummy()),),
                        TypeDefinition::new(
                            "Bar",
                            types::Reference::new("Foo", SourceInformation::dummy()),
                        )
                    ],
                    vec![],
                )),
                Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![],
                    vec![
                        TypeDefinition::new("Foo", types::Number::new(SourceInformation::dummy()),),
                        TypeDefinition::new("Bar", types::Number::new(SourceInformation::dummy()),)
                    ],
                    vec![],
                )
            );
        }

        #[test]
        fn remove_reference_type_in_definition() {
            assert_eq!(
                remove_reference_types(&Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![],
                    vec![TypeDefinition::new(
                        "Foo",
                        types::Number::new(SourceInformation::dummy()),
                    )],
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Reference::new("Foo", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                )),
                Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![],
                    vec![TypeDefinition::new(
                        "Foo",
                        types::Number::new(SourceInformation::dummy()),
                    )],
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                )
            );
        }
    }

    mod external_types {
        use super::*;

        fn create_module_interface() -> ModuleInterface {
            ModuleInterface::new(
                ModulePath::new(Package::new("Module", ""), vec![]),
                vec![(
                    "Foo".into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
                Default::default(),
            )
        }

        #[test]
        fn remove_reference_type_in_type_definition() {
            assert_eq!(
                remove_reference_types(&Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![create_module_interface()],
                    vec![TypeDefinition::new(
                        "Bar",
                        types::Reference::new("Module.Foo", SourceInformation::dummy()),
                    )],
                    vec![],
                )),
                Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![create_module_interface()],
                    vec![TypeDefinition::new(
                        "Bar",
                        types::Number::new(SourceInformation::dummy()),
                    )],
                    vec![],
                )
            );
        }

        #[test]
        fn remove_reference_type_in_definition() {
            assert_eq!(
                remove_reference_types(&Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![create_module_interface()],
                    vec![],
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Reference::new("Module.Foo", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                )),
                Module::new(
                    ModulePath::new(Package::new("", ""), vec![]),
                    Export::new(Default::default()),
                    vec![create_module_interface()],
                    vec![],
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()],
                )
            );
        }
    }
}
