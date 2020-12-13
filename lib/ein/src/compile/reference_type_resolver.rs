use super::error::CompileError;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct ReferenceTypeResolver {
    environment: HashMap<String, Type>,
}

impl ReferenceTypeResolver {
    pub fn new(module: &Module) -> Arc<Self> {
        Self {
            environment: module
                .imports()
                .iter()
                .flat_map(|import| {
                    import
                        .module_interface()
                        .types()
                        .iter()
                        .map(|(name, type_)| (name.into(), type_.clone()))
                })
                .chain(
                    module
                        .ffi_imports()
                        .iter()
                        .flat_map(|interface| interface.types().clone()),
                )
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
            Type::Any(_)
            | Type::Boolean(_)
            | Type::Function(_)
            | Type::List(_)
            | Type::None(_)
            | Type::Number(_)
            | Type::Record(_)
            | Type::String(_)
            | Type::Union(_)
            | Type::Unknown(_)
            | Type::Variable(_) => Ok(type_.clone()),
        }
    }

    pub fn resolve_to_any(&self, type_: &Type) -> Result<Option<types::Any>, CompileError> {
        Ok(match self.resolve(type_)? {
            Type::Any(any) => Some(any),
            _ => None,
        })
    }

    pub fn resolve_to_function(
        &self,
        type_: &Type,
    ) -> Result<Option<types::Function>, CompileError> {
        Ok(match self.resolve(type_)? {
            Type::Function(function) => Some(function),
            _ => None,
        })
    }

    pub fn resolve_to_list(&self, type_: &Type) -> Result<Option<types::List>, CompileError> {
        Ok(match self.resolve(type_)? {
            Type::List(list) => Some(list),
            _ => None,
        })
    }

    pub fn resolve_to_record(&self, type_: &Type) -> Result<Option<types::Record>, CompileError> {
        Ok(match self.resolve(type_)? {
            Type::Record(record) => Some(record),
            _ => None,
        })
    }

    pub fn is_any(&self, type_: &Type) -> Result<bool, CompileError> {
        Ok(matches!(self.resolve(type_)?, Type::Any(_)))
    }

    pub fn is_function(&self, type_: &Type) -> Result<bool, CompileError> {
        Ok(matches!(self.resolve(type_)?, Type::Function(_)))
    }

    pub fn is_list(&self, type_: &Type) -> Result<bool, CompileError> {
        Ok(matches!(self.resolve(type_)?, Type::List(_)))
    }

    pub fn is_union(&self, type_: &Type) -> Result<bool, CompileError> {
        Ok(matches!(self.resolve(type_)?, Type::Union(_)))
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
                vec![Import::new(
                    ModuleInterface::new(
                        ModulePath::new(Package::new("Foo", ""), vec![]),
                        Default::default(),
                        vec![(
                            "Foo".into(),
                            types::Number::new(SourceInformation::dummy()).into()
                        )]
                        .drain(..)
                        .collect(),
                        Default::default(),
                        Default::default(),
                    ),
                    true
                )],
                vec![],
                vec![],
                vec![],
            ))
            .resolve(&types::Reference::new("Foo", SourceInformation::dummy()).into()),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn resolve_type_in_ffi_import() {
        assert_eq!(
            ReferenceTypeResolver::new(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![BuiltinInterface::new(
                    vec![(
                        "Foo".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    )]
                    .into_iter()
                    .collect(),
                    Default::default()
                )],
                vec![],
                vec![],
            ))
            .resolve(&types::Reference::new("Foo", SourceInformation::dummy()).into()),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }
}
