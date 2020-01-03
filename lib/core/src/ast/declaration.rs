use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    name: String,
    type_: Type,
}

impl Declaration {
    pub fn new(name: impl Into<String>, type_: impl Into<Type>) -> Self {
        Self {
            name: name.into(),
            type_: type_.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn convert_types(&self, convert: &impl Fn(&Type) -> Type) -> Self {
        Self {
            name: self.name.clone(),
            type_: convert(&self.type_),
        }
    }
}
