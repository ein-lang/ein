use lang::debug::SourceInformation;
use lang::types;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref BUILTIN_CONFIGURATION: Arc<lang::BuiltinConfiguration> =
        lang::BuiltinConfiguration {
            functions: vec![
                (
                    "_ein_join_strings".into(),
                    types::Function::new(
                        types::EinString::new(SourceInformation::builtin()),
                        types::Function::new(
                            types::EinString::new(SourceInformation::builtin()),
                            types::EinString::new(SourceInformation::builtin()),
                            SourceInformation::builtin(),
                        ),
                        SourceInformation::builtin(),
                    ),
                ),
                (
                    "_ein_slice_string".into(),
                    types::Function::new(
                        types::EinString::new(SourceInformation::builtin()),
                        types::Function::new(
                            types::Number::new(SourceInformation::builtin()),
                            types::Function::new(
                                types::Number::new(SourceInformation::builtin()),
                                types::EinString::new(SourceInformation::builtin()),
                                SourceInformation::builtin(),
                            ),
                            SourceInformation::builtin(),
                        ),
                        SourceInformation::builtin(),
                    ),
                ),
                (
                    "_ein_number_to_string".into(),
                    types::Function::new(
                        types::Number::new(SourceInformation::builtin()),
                        types::EinString::new(SourceInformation::builtin()),
                        SourceInformation::builtin(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        }
        .into();
}
