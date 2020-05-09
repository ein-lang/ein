use super::global_name_renamer::GlobalNameRenamer;
use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct GlobalNameQualifier {
    global_name_renamer: GlobalNameRenamer,
}

impl GlobalNameQualifier {
    pub fn new(module: &Module, excluded_names: &HashSet<String>) -> Self {
        let mut names = HashMap::new();

        for imported_module in module.imported_modules() {
            for name in imported_module.variables().keys() {
                names.insert(
                    imported_module.path().qualify_name(name),
                    imported_module.path().fully_qualify_name(name),
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

        for name in excluded_names {
            names.remove(name);
        }

        Self {
            global_name_renamer: GlobalNameRenamer::new(names),
        }
    }

    pub fn qualify(&self, module: &Module) -> Module {
        self.global_name_renamer.rename(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn qualify_names_in_value_definitions() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![ast::ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameQualifier::new(&module, &Default::default()).qualify(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![ast::ValueDefinition::new(
                    "M().x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]
            )
        );
    }

    #[test]
    fn qualify_names_in_imported_modules() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![ast::ModuleInterface::new(
                ModulePath::new(Package::new("A", ""), vec!["B".into()]),
                Default::default(),
                vec![(
                    "y".into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                )]
                .into_iter()
                .collect(),
            )],
            vec![],
            vec![ast::ValueDefinition::new(
                "x",
                Variable::new("B.y", SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameQualifier::new(&module, &Default::default()).qualify(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![ast::ModuleInterface::new(
                    ModulePath::new(Package::new("A", ""), vec!["B".into()]),
                    Default::default(),
                    vec![(
                        "y".into(),
                        types::Number::new(SourceInformation::dummy()).into(),
                    )]
                    .into_iter()
                    .collect(),
                )],
                vec![],
                vec![ast::ValueDefinition::new(
                    "M().x",
                    Variable::new("A().B.y", SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()],
            )
        );
    }

    #[test]
    fn do_not_qualify_excluded_names() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![ast::ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameQualifier::new(&module, &vec!["x".into()].into_iter().collect())
                .qualify(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![ast::ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]
            )
        );
    }

    #[test]
    fn qualify_names_in_type_definitions() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![TypeDefinition::new(
                "x",
                types::None::new(SourceInformation::dummy()),
            )],
            vec![],
        );

        assert_eq!(
            GlobalNameQualifier::new(&module, &Default::default()).qualify(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "M().x",
                    types::None::new(SourceInformation::dummy()),
                )],
                vec![],
            )
        );
    }
}
