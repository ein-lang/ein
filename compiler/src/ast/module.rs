use super::function_definition::FunctionDefinition;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    function_definitions: Vec<FunctionDefinition>,
}

impl Module {
    pub fn new(function_definitions: Vec<FunctionDefinition>) -> Self {
        Self {
            function_definitions,
        }
    }

    pub fn function_definitions(&self) -> &[FunctionDefinition] {
        &self.function_definitions
    }
}
