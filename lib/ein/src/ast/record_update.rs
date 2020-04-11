use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordUpdate {
    type_: types::Reference,
    argument: Box<Expression>,
    elements: BTreeMap<String, Expression>,
    source_information: Rc<SourceInformation>,
}

impl RecordUpdate {
    pub fn new(
        type_: types::Reference,
        argument: impl Into<Expression>,
        elements: BTreeMap<String, Expression>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            type_,
            argument: Box::new(argument.into()),
            elements,
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &types::Reference {
        &self.type_
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn elements(&self) -> &BTreeMap<String, Expression> {
        &self.elements
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            self.argument.convert_expressions(convert)?,
            self.elements
                .iter()
                .map(|(name, expression)| {
                    Ok((name.into(), expression.convert_expressions(convert)?))
                })
                .collect::<Result<_, _>>()?,
            self.source_information.clone(),
        ))
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.type_.clone(),
            self.argument.convert_types(convert),
            self.elements
                .iter()
                .map(|(name, expression)| (name.into(), expression.convert_types(convert)))
                .collect(),
            self.source_information.clone(),
        )
    }
}
