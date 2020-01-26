use crate::ast;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NameQualifier {
    names: HashMap<String, String>,
}

// NameQualifier is not meant to qualify names in the original modules but
// names in their outputs on compilation.
impl NameQualifier {
    pub fn new(module: &ast::Module, special_names: HashMap<String, String>) -> Self {
        let mut names = HashMap::new();

        for imported_module in module.imported_modules() {
            for name in imported_module.variables().keys() {
                names.insert(
                    imported_module.path().qualify_name(name),
                    imported_module.path().fully_qualify_name(name),
                );
            }
        }

        for definition in module.definitions() {
            names.insert(
                definition.name().into(),
                module.path().fully_qualify_name(definition.name()),
            );
        }

        names.extend(special_names);

        Self { names }
    }

    pub fn qualify_core_module(&self, module: &ssf::ir::Module) -> ssf::ir::Module {
        module.rename_global_variables(&self.names)
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

    #[test]
    fn qualify_names_in_target_module() {
        assert_eq!(
            NameQualifier::new(
                &ast::Module::new(
                    ModulePath::new(Package::new("M", ""), vec![]),
                    ast::Export::new(Default::default()),
                    vec![],
                    vec![],
                    vec![ast::ValueDefinition::new(
                        "x",
                        ast::Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()]
                ),
                HashMap::new()
            )
            .qualify_core_module(
                &ssf::ir::Module::new(
                    vec![],
                    vec![ssf::ir::ValueDefinition::new(
                        "x",
                        ssf::ir::Expression::Number(42.0),
                        ssf::types::Value::Number
                    )
                    .into()]
                )
                .unwrap()
            ),
            ssf::ir::Module::new(
                vec![],
                vec![ssf::ir::ValueDefinition::new(
                    "M().x",
                    ssf::ir::Expression::Number(42.0),
                    ssf::types::Value::Number
                )
                .into()]
            )
            .unwrap()
        );
    }

    #[test]
    fn qualify_names_in_imported_modules() {
        assert_eq!(
            NameQualifier::new(
                &ast::Module::new(
                    ModulePath::new(Package::new("M", ""), vec![]),
                    ast::Export::new(Default::default()),
                    vec![ast::ModuleInterface::new(
                        ModulePath::new(Package::new("A", ""), vec!["B".into()]),
                        Default::default(),
                        vec![(
                            "y".into(),
                            types::Number::new(SourceInformation::dummy()).into()
                        )]
                        .into_iter()
                        .collect()
                    )],
                    vec![],
                    vec![ast::ValueDefinition::new(
                        "x",
                        ast::Variable::new("B.y", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into()]
                ),
                HashMap::new()
            )
            .qualify_core_module(
                &ssf::ir::Module::new(
                    vec![ssf::ir::Declaration::new("B.y", ssf::types::Value::Number)],
                    vec![ssf::ir::ValueDefinition::new(
                        "x",
                        ssf::ir::Variable::new("B.y"),
                        ssf::types::Value::Number
                    )
                    .into()]
                )
                .unwrap()
            ),
            ssf::ir::Module::new(
                vec![ssf::ir::Declaration::new(
                    "A().B.y",
                    ssf::types::Value::Number
                )],
                vec![ssf::ir::ValueDefinition::new(
                    "M().x",
                    ssf::ir::Variable::new("A().B.y"),
                    ssf::types::Value::Number
                )
                .into()]
            )
            .unwrap()
        );
    }
}
