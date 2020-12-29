use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref COMPILE_CONFIGURATION: Arc<lang::CompileConfiguration> =
        lang::CompileConfiguration {
            malloc_function_name: "ein_malloc".into(),
            panic_function_name: "ein_panic".into(),
            list_type_configuration: lang::ListTypeConfiguration {
                empty_list_variable_name: "_emptyList".into(),
                concatenate_function_name: "_concatenateLists".into(),
                equal_function_name: "_equalLists".into(),
                prepend_function_name: "_prependToList".into(),
                deconstruct_function_name: "_firstRest".into(),
                first_function_name: "_first".into(),
                rest_function_name: "_rest".into(),
                list_type_name: "_AnyList".into(),
                first_rest_type_name: "_FirstRest".into(),
                map_function_name: "_mapList".into(),
            }
            .into(),
            string_type_configuration: lang::StringTypeConfiguration {
                equal_function_name: "_ein_equal_strings".into(),
            }
            .into(),
            error_type_configuration: lang::ErrorTypeConfiguration {
                error_type_name: "Error".into(),
            }
            .into(),
            main_module_configuration: Some(
                lang::MainModuleConfiguration {
                    source_main_function_name: "main".into(),
                    object_main_function_name: "ein_main".into(),
                    argument_type_name: "System".into(),
                }
                .into()
            ),
        }
        .into();
}
