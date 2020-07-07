use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCoercion {
    argument: Arc<Expression>,
    from: Type,
    to: Type,
    source_information: Arc<SourceInformation>,
}

impl TypeCoercion {
    pub fn new(
        argument: impl Into<Expression>,
        from: impl Into<Type>,
        to: impl Into<Type>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            argument: Arc::new(argument.into()),
            from: from.into(),
            to: to.into(),
            source_information: source_information.into(),
        }
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn from(&self) -> &Type {
        &self.from
    }

    pub fn to(&self) -> &Type {
        &self.to
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.argument.convert_expressions(convert)?,
            self.from.clone(),
            self.to.clone(),
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.argument.convert_types(convert)?,
            self.from.convert_types(convert)?,
            self.to.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
