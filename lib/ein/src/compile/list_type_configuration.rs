use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    pub static ref LIST_TYPE_CONFIGURATION: Arc<ListTypeConfiguration> =
        ListTypeConfiguration::new(
            "emptyList",
            "concatenateLists",
            "equalLists",
            "prependToLists",
            "deconstruct",
            "first",
            "rest",
            "GenericList",
            "FirstRest",
        )
        .into();
}

pub struct ListTypeConfiguration {
    empty_list_variable_name: String,
    concatenate_function_name: String,
    equal_function_name: String,
    prepend_function_name: String,
    deconstruct_function_name: String,
    first_function_name: String,
    rest_function_name: String,
    list_type_name: String,
    first_rest_type_name: String,
}

impl ListTypeConfiguration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        empty_list_variable_name: impl Into<String>,
        concatenate_function_name: impl Into<String>,
        equal_function_name: impl Into<String>,
        prepend_function_name: impl Into<String>,
        deconstruct_function_name: impl Into<String>,
        first_function_name: impl Into<String>,
        rest_function_name: impl Into<String>,
        list_type_name: impl Into<String>,
        first_rest_type_name: impl Into<String>,
    ) -> Self {
        Self {
            empty_list_variable_name: empty_list_variable_name.into(),
            concatenate_function_name: concatenate_function_name.into(),
            equal_function_name: equal_function_name.into(),
            prepend_function_name: prepend_function_name.into(),
            deconstruct_function_name: deconstruct_function_name.into(),
            first_function_name: first_function_name.into(),
            rest_function_name: rest_function_name.into(),
            list_type_name: list_type_name.into(),
            first_rest_type_name: first_rest_type_name.into(),
        }
    }

    pub fn empty_list_variable_name(&self) -> &str {
        &self.empty_list_variable_name
    }

    pub fn concatenate_function_name(&self) -> &str {
        &self.concatenate_function_name
    }

    pub fn equal_function_name(&self) -> &str {
        &self.equal_function_name
    }

    pub fn prepend_function_name(&self) -> &str {
        &self.prepend_function_name
    }

    pub fn deconstruct_function_name(&self) -> &str {
        &self.deconstruct_function_name
    }

    pub fn first_function_name(&self) -> &str {
        &self.first_function_name
    }

    pub fn rest_function_name(&self) -> &str {
        &self.rest_function_name
    }

    pub fn list_type_name(&self) -> &str {
        &self.list_type_name
    }

    pub fn first_rest_type_name(&self) -> &str {
        &self.first_rest_type_name
    }

    pub fn qualify(&self, names: &HashMap<String, String>) -> Self {
        Self {
            empty_list_variable_name: self.qualify_name(&self.empty_list_variable_name, &names),
            concatenate_function_name: self.qualify_name(&self.concatenate_function_name, &names),
            equal_function_name: self.qualify_name(&self.equal_function_name, &names),
            prepend_function_name: self.qualify_name(&self.prepend_function_name, &names),
            deconstruct_function_name: self.qualify_name(&self.deconstruct_function_name, &names),
            first_function_name: self.qualify_name(&self.first_function_name, &names),
            rest_function_name: self.qualify_name(&self.rest_function_name, &names),
            list_type_name: self.qualify_name(&self.list_type_name, &names),
            first_rest_type_name: self.qualify_name(&self.first_rest_type_name, &names),
        }
    }

    fn qualify_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
