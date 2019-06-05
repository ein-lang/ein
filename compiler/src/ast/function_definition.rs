use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<String>,
    body: Expression,
}

impl FunctionDefinition {
    pub fn new(name: String, arguments: Vec<String>, body: Expression) -> Self {
        Self {
            name,
            arguments,
            body,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }
}
