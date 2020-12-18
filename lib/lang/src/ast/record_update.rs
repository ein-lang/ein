use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordUpdate {
    type_: Type,
    argument: Arc<Expression>,
    elements: BTreeMap<String, Expression>,
    source_information: Arc<SourceInformation>,
}

impl RecordUpdate {
    pub fn new(
        type_: impl Into<Type>,
        argument: impl Into<Expression>,
        elements: BTreeMap<String, Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            argument: Arc::new(argument.into()),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn elements(&self) -> &BTreeMap<String, Expression> {
        &self.elements
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            self.argument.transform_expressions(transform)?,
            self.elements
                .iter()
                .map(|(name, expression)| {
                    Ok((name.into(), expression.transform_expressions(transform)?))
                })
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.transform_types(transform)?,
            self.argument.transform_types(transform)?,
            self.elements
                .iter()
                .map(|(name, expression)| Ok((name.into(), expression.transform_types(transform)?)))
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }
}
