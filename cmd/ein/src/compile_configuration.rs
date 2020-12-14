use ein::debug::SourceInformation;
use ein::types;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref COMPILE_CONFIGURATION: Arc<app::CompileConfiguration> =
        app::CompileConfiguration {
            source_main_function_name: "main".into(),
            object_main_function_name: "ein_main".into(),
            malloc_function_name: "ein_malloc".into(),
            panic_function_name: "ein_panic".into(),
            list_type_configuration: app::ListTypeConfiguration {
                empty_list_variable_name: "_emptyList".into(),
                concatenate_function_name: "_concatenateLists".into(),
                equal_function_name: "_equalLists".into(),
                prepend_function_name: "_prependToList".into(),
                deconstruct_function_name: "_firstRest".into(),
                first_function_name: "_first".into(),
                rest_function_name: "_rest".into(),
                list_type_name: "_AnyList".into(),
                first_rest_type_name: "_FirstRest".into(),
            }
            .into(),
            string_type_configuration: app::StringTypeConfiguration {
                equal_function_name: "_ein_equal_strings".into(),
            }
            .into(),
            builtin_configuration: Arc::new(ein::BuiltinConfiguration {
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
                ]
                .into_iter()
                .collect(),
            }),
        }
        .into();
}
