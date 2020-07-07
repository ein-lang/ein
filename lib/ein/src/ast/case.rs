use super::alternative::Alternative;
use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Case {
    type_: Type,
    name: String,
    argument: Arc<Expression>,
    alternatives: Vec<Alternative>,
    source_information: Arc<SourceInformation>,
}

impl Case {
    pub fn new(
        name: impl Into<String>,
        argument: impl Into<Expression>,
        alternatives: Vec<Alternative>,
        source_information: impl Into<Arc<SourceInformation>> + Clone,
    ) -> Self {
        Self {
            type_: types::Unknown::new(source_information.clone()).into(),
            name: name.into(),
            argument: Arc::new(argument.into()),
            alternatives,
            source_information: source_information.into(),
        }
    }

    pub fn with_type(
        type_: impl Into<Type>,
        name: impl Into<String>,
        argument: impl Into<Expression>,
        alternatives: Vec<Alternative>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            name: name.into(),
            argument: Arc::new(argument.into()),
            alternatives,
            source_information: source_information.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn alternatives(&self) -> &[Alternative] {
        &self.alternatives
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            type_: self.type_.clone(),
            name: self.name.clone(),
            argument: self.argument.convert_expressions(convert)?.into(),
            alternatives: self
                .alternatives
                .iter()
                .map(|alternative| alternative.convert_expressions(convert))
                .collect::<Result<_, _>>()?,
            source_information: self.source_information.clone(),
        })
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            type_: self.type_.convert_types(convert)?,
            name: self.name.clone(),
            argument: self.argument.convert_types(convert)?.into(),
            alternatives: self
                .alternatives
                .iter()
                .map(|alternative| alternative.convert_types(convert))
                .collect::<Result<_, _>>()?,
            source_information: self.source_information.clone(),
        })
    }
}
