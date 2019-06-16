use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    name: String,
    type_: Type,
}

impl Argument {
    pub fn new(name: String, type_: Type) -> Self {
        Self { name, type_ }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }
}
