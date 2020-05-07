use super::error::CompileError;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct ReferenceTypeResolver {
    environment: HashMap<String, Type>,
}

impl ReferenceTypeResolver {
    pub fn new(module: &Module) -> Rc<Self> {
        Self {
            environment: module
                .imported_modules()
                .iter()
                .flat_map(|imported_module| {
                    imported_module.types().iter().map(move |(name, type_)| {
                        (imported_module.path().qualify_name(name), type_.clone())
                    })
                })
                .chain(module.type_definitions().iter().map(|type_definition| {
                    (
                        type_definition.name().into(),
                        type_definition.type_().clone(),
                    )
                }))
                .collect(),
        }
        .into()
    }

    pub fn resolve_reference(&self, reference: &types::Reference) -> Result<Type, CompileError> {
        self.resolve(
            self.environment
                .get(reference.name())
                .ok_or_else(|| CompileError::TypeNotFound(reference.clone()))?,
        )
    }

    pub fn resolve(&self, type_: &Type) -> Result<Type, CompileError> {
        match type_ {
            Type::Reference(reference) => self.resolve_reference(reference),
            Type::Boolean(_)
            | Type::Function(_)
            | Type::None(_)
            | Type::Number(_)
            | Type::Record(_)
            | Type::Union(_) => Ok(type_.clone()),
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
            Ok(types::Number::new(SourceInformation::dummy()).into())
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
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }
}
