use super::expression::Expression;
use super::Argument;
use crate::types;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    arguments: Vec<Argument>,
    body: Expression,
    result_type: types::Value,
    type_: types::Function,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        arguments: Vec<Argument>,
        body: Expression,
        result_type: types::Value,
    ) -> Self {
        let type_ = types::Function::new(
            arguments
                .iter()
                .map(|argument| argument.type_().clone())
                .collect::<Vec<_>>(),
            result_type.clone(),
        );

        Self {
            name,
            arguments,
            body,
            result_type,
            type_,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn body(&self) -> &Expression {
        &self.body
    }

    pub fn result_type(&self) -> &types::Value {
        &self.result_type
    }

    pub fn type_(&self) -> &types::Function {
        &self.type_
    }
}
