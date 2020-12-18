#[cfg(test)]
use lazy_static::lazy_static;
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
lazy_static! {
    pub static ref LIST_TYPE_CONFIGURATION: Arc<ListTypeConfiguration> = ListTypeConfiguration {
        empty_list_variable_name: "emptyList".into(),
        concatenate_function_name: "concatenateLists".into(),
        equal_function_name: "equalLists".into(),
        prepend_function_name: "prependToLists".into(),
        deconstruct_function_name: "deconstruct".into(),
        first_function_name: "first".into(),
        rest_function_name: "rest".into(),
        list_type_name: "GenericList".into(),
        first_rest_type_name: "FirstRest".into(),
    }
    .into();
}

pub struct ListTypeConfiguration {
    pub empty_list_variable_name: String,
    pub concatenate_function_name: String,
    pub equal_function_name: String,
    pub prepend_function_name: String,
    pub deconstruct_function_name: String,
    pub first_function_name: String,
    pub rest_function_name: String,
    pub list_type_name: String,
    pub first_rest_type_name: String,
}

impl ListTypeConfiguration {
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
