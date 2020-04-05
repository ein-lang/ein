use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ReferenceTypeResolver {
    environment: HashMap<String, Type>,
}

impl ReferenceTypeResolver {
    pub fn new(module: &Module) -> Self {
        Self {
            environment: module
                .imported_modules()
                .iter()
                .map(move |imported_module| {
                    imported_module.types().iter().map(move |(name, type_)| {
                        (imported_module.path().qualify_name(name), type_.clone())
                    })
                })
                .flatten()
                .chain(module.type_definitions().iter().map(|type_definition| {
                    (
                        type_definition.name().into(),
                        type_definition.type_().clone(),
                    )
                }))
                .collect(),
        }
    }

    pub fn resolve_reference(&self, reference: &types::Reference) -> Type {
        self.resolve(&self.environment[reference.name()])
    }

    pub fn resolve(&self, type_: &Type) -> Type {
        match type_ {
            Type::Function(function) => types::Function::new(
                self.resolve(function.argument()),
                self.resolve(function.result()),
                function.source_information().clone(),
            )
            .into(),
            Type::Record(record) => types::Record::new(
                record.name(),
                record
                    .elements()
                    .iter()
                    .map(|(name, type_)| (name.into(), self.resolve(type_)))
                    .collect(),
                record.source_information().clone(),
            )
            .into(),
            Type::Reference(reference) => self.resolve_reference(reference),
            Type::Boolean(_) | Type::None(_) | Type::Number(_) => type_.clone(),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use crate::types;

    #[test]
    fn resolve_resolved_type() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::dummy())
                .resolve(&types::Number::new(SourceInformation::dummy()).into()),
            types::Number::new(SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn resolve_type_in_imported_module() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![ModuleInterface::new(
                    ModulePath::new(Package::new("Foo", ""), vec![]),
                    vec![(
                        "Foo".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    )]
                    .drain(..)
                    .collect(),
                    Default::default()
                )],
                vec![],
                vec![],
            ))
            .resolve(&types::Reference::new("Foo.Foo", SourceInformation::dummy()).into()),
            types::Number::new(SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn resolve_to_number_type() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "Foo",
                    types::Number::new(SourceInformation::dummy()),
                )],
                vec![],
            ))
            .resolve(&types::Reference::new("Foo", SourceInformation::dummy()).into()),
            types::Number::new(SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn resolve_to_function_type() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "Foo",
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![],
            ))
            .resolve(&types::Reference::new("Foo", SourceInformation::dummy()).into()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        );
    }

    #[test]
    fn resolve_function_results_recursively() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![
                    TypeDefinition::new(
                        "Foo",
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                    ),
                    TypeDefinition::new(
                        "Bar",
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Reference::new("Foo", SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                    )
                ],
                vec![],
            ))
            .resolve(&types::Reference::new("Bar", SourceInformation::dummy()).into()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()
        );
    }
}
