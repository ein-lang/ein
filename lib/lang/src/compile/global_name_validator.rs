use super::error::CompileError;
use crate::ast::*;
use crate::debug::SourceInformation;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GlobalNameValidator {}

impl GlobalNameValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate(&self, module: &Module) -> Result<(), CompileError> {
        let mut names = HashMap::<&str, Arc<SourceInformation>>::new();

        for declaration in module.import_foreigns() {
            if let Some(source_information) = names.get(declaration.name()) {
                return Err(CompileError::DuplicateNames(
                    source_information.clone(),
                    declaration.source_information().clone(),
                ));
            }

            names.insert(declaration.name(), declaration.source_information().clone());
        }

        for definition in module.definitions() {
            if let Some(source_information) = names.get(definition.name()) {
                return Err(CompileError::DuplicateNames(
                    source_information.clone(),
                    definition.source_information().clone(),
                ));
            }

            names.insert(definition.name(), definition.source_information().clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::ModulePath;
    use crate::types;

    #[test]
    fn validate_duplicate_names() {
        assert_eq!(
            GlobalNameValidator::new().validate(&Module::from_definitions(vec![
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
                VariableDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ])),
            Err(CompileError::DuplicateNames(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn validate_duplicate_names_with_import_foreigns() {
        assert_eq!(
            GlobalNameValidator::new().validate(&Module::new(
                ModulePath::dummy(),
                Export::new(Default::default()),
                vec![],
                vec![ImportForeign::new(
                    "foo",
                    "foo",
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )],
                vec![],
                vec![VariableDefinition::new(
                    "foo",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]
            )),
            Err(CompileError::DuplicateNames(
                SourceInformation::dummy().into(),
                SourceInformation::dummy().into()
            ))
        );
    }
}
