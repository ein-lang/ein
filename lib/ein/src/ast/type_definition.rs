use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDefinition {
    name: String,
    type_: Type,
}

impl TypeDefinition {
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

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.name.clone(),
            self.type_.transform_types(transform)?,
        ))
    }
}
