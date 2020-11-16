use crate::ast::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GlobalNameMapCreator {}

impl GlobalNameMapCreator {
    pub fn create(module: &Module) -> Arc<HashMap<String, String>> {
        let mut names = HashMap::new();

        for import in module.imports() {
            for name in import.module_interface().exported_names() {
                names.insert(
                    if import.qualified() {
                        import.module_interface().path().qualify_name(name)
                    } else {
                        name.into()
                    },
                    import.module_interface().path().fully_qualify_name(name),
                );
            }
        }

        for type_definition in module.type_definitions() {
            names.insert(
                type_definition.name().into(),
                module.path().fully_qualify_name(type_definition.name()),
            );
        }

        for definition in module.definitions() {
            names.insert(
                definition.name().into(),
                module.path().fully_qualify_name(definition.name()),
            );
        }

        names.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_name_map_from_variable_definition() {
        assert_eq!(
            GlobalNameMapCreator::create(&Module::from_definitions(vec![VariableDefinition::new(
                "x",
                None::new(SourceInformation::dummy()),
                types::None::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()])),
            vec![("x".into(), "().x".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
                .into()
        );
    }

    #[test]
    fn create_name_map_from_function_definition() {
        assert_eq!(
            GlobalNameMapCreator::create(&Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["x".into()],
                None::new(SourceInformation::dummy()),
                types::Function::new(
                    types::None::new(SourceInformation::dummy()),
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()]),),
            vec![("f".into(), "().f".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
                .into()
        );
    }

    #[test]
    fn create_name_map_from_type_definition() {
        assert_eq!(
            GlobalNameMapCreator::create(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "x",
                    types::None::new(SourceInformation::dummy())
                )],
                vec![]
            ),),
            vec![("x".into(), "().x".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
                .into()
        );
    }

    #[test]
    fn create_name_map_from_qualified_import() {
        assert_eq!(
            GlobalNameMapCreator::create(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(
                    ModuleInterface::new(
                        ModulePath::new(Package::new("p", ""), vec!["m".into()]),
                        vec!["x".into()].into_iter().collect(),
                        Default::default(),
                        vec![(
                            "x".into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                    true,
                )],
                vec![],
                vec![]
            )),
            vec![("m.x".into(), "p().m.x".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
                .into()
        );
    }

    #[test]
    fn create_name_map_from_unqualified_import() {
        assert_eq!(
            GlobalNameMapCreator::create(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![Import::new(
                    ModuleInterface::new(
                        ModulePath::new(Package::new("p", ""), vec!["m".into()]),
                        vec!["x".into()].into_iter().collect(),
                        Default::default(),
                        vec![(
                            "x".into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                    false,
                )],
                vec![],
                vec![]
            )),
            vec![("x".into(), "p().m.x".into())]
                .into_iter()
                .collect::<HashMap<_, _>>()
                .into()
        );
    }
}
