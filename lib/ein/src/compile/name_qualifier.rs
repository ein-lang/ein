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

    pub fn qualify_core_module(&self, module: &core::ast::Module) -> core::ast::Module {
        core::ast::Module::new(
            module.declarations().to_vec(),
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    core::ast::Definition::FunctionDefinition(function_definition) => {
                        core::ast::FunctionDefinition::new(
                            self.names
                                .get(function_definition.name())
                                .cloned()
                                .unwrap_or_else(|| function_definition.name().into()),
                            function_definition.environment().to_vec(),
                            function_definition.arguments().to_vec(),
                            function_definition.body().rename_variables(&self.names),
                            function_definition.result_type().clone(),
                        )
                        .into()
                    }
                    core::ast::Definition::ValueDefinition(value_definition) => {
                        core::ast::ValueDefinition::new(
                            self.names
                                .get(value_definition.name())
                                .cloned()
                                .unwrap_or_else(|| value_definition.name().into()),
                            value_definition.body().rename_variables(&self.names),
                            value_definition.type_().clone(),
                        )
                        .into()
                    }
                })
                .collect(),
        )
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
            .qualify_core_module(&core::ast::Module::new(
                vec![],
                vec![core::ast::ValueDefinition::new(
                    "x",
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number
                )
                .into()]
            )),
            core::ast::Module::new(
                vec![],
                vec![core::ast::ValueDefinition::new(
                    "M().x",
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number
                )
                .into()]
            )
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
            .qualify_core_module(&core::ast::Module::new(
                vec![],
                vec![core::ast::ValueDefinition::new(
                    "x",
                    core::ast::Variable::new("B.y"),
                    core::types::Value::Number
                )
                .into()]
            )),
            core::ast::Module::new(
                vec![],
                vec![core::ast::ValueDefinition::new(
                    "M().x",
                    core::ast::Variable::new("A().B.y"),
                    core::types::Value::Number
                )
                .into()]
            )
        );
    }
}
