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
}
