pub struct ListLiteralConfiguration {
    empty_list_variable_name: String,
    concatenate_function_name: String,
    equal_function_name: String,
    prepend_function_name: String,
    list_type_name: String,
}

impl ListLiteralConfiguration {
    pub fn new(
        empty_list_variable_name: impl Into<String>,
        concatenate_function_name: impl Into<String>,
        equal_function_name: impl Into<String>,
        prepend_function_name: impl Into<String>,
        list_type_name: impl Into<String>,
    ) -> Self {
        Self {
            empty_list_variable_name: empty_list_variable_name.into(),
            concatenate_function_name: concatenate_function_name.into(),
            equal_function_name: equal_function_name.into(),
            prepend_function_name: prepend_function_name.into(),
            list_type_name: list_type_name.into(),
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

    pub fn list_type_name(&self) -> &str {
        &self.list_type_name
    }
}
