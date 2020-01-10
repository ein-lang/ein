use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct ReferenceTypeResolver {
    environment: HashMap<String, Type>,
}

impl ReferenceTypeResolver {
    pub fn new(module: &Module) -> Self {
        Self {
            environment: module
                .type_definitions()
                .iter()
                .map(|type_definition| {
                    (
                        type_definition.name().into(),
                        type_definition.type_().clone(),
                    )
                })
                .collect(),
        }
    }

    pub fn resolve(&self, type_: &Type) -> Type {
        match type_ {
            Type::Function(function) => self.resolve_function(function).into(),
            Type::Number(_) => type_.clone(),
            Type::Reference(reference) => self.resolve(&self.environment[reference.name()]),
            Type::Unknown(_) => unreachable!(),
            Type::Variable(_) => unreachable!(),
        }
    }

    fn resolve_function(&self, function: &types::Function) -> types::Function {
        types::Function::new(
            self.resolve(function.argument()),
            self.resolve(function.result()),
            function.source_information().clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;

    #[test]
    fn resolve_resolved_type() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::dummy())
                .resolve(&types::Number::new(SourceInformation::dummy()).into()),
            types::Number::new(SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn resolve_function_argument() {
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
            .resolve(
                &types::Function::new(
                    types::Reference::new("Foo", SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        );
    }

    #[test]
    fn resolve_function_result() {
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
            .resolve(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Reference::new("Foo", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        );
    }
}
