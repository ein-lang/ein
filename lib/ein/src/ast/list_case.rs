use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct ListCase {
    argument: Arc<Expression>,
    head_name: String,
    tail_name: String,
    emtpy_alternative: Arc<Expression>,
    non_emtpy_alternative: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl ListCase {
    pub fn new(
        argument: impl Into<Expression>,
        head_name: impl Into<String>,
        tail_name: impl Into<String>,
        emtpy_alternative: impl Into<Expression>,
        non_emtpy_alternative: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>> + Clone,
    ) -> Self {
        Self {
            argument: Arc::new(argument.into()),
            head_name: head_name.into(),
            tail_name: tail_name.into(),
            emtpy_alternative: Arc::new(emtpy_alternative.into()),
            non_emtpy_alternative: Arc::new(non_emtpy_alternative.into()),
            source_information: source_information.into(),
        }
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn head_name(&self) -> &str {
        &self.head_name
    }

    pub fn tail_name(&self) -> &str {
        &self.tail_name
    }

    pub fn emtpy_alternative(&self) -> &Expression {
        &self.emtpy_alternative
    }

    pub fn non_emtpy_alternative(&self) -> &Expression {
        &self.non_emtpy_alternative
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            argument: self.argument.transform_expressions(transform)?.into(),
            head_name: self.head_name.clone(),
            tail_name: self.tail_name.clone(),
            emtpy_alternative: self
                .emtpy_alternative
                .transform_expressions(transform)?
                .into(),
            non_emtpy_alternative: self
                .non_emtpy_alternative
                .transform_expressions(transform)?
                .into(),
            source_information: self.source_information.clone(),
        })
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self {
            argument: self.argument.transform_types(transform)?.into(),
            head_name: self.head_name.clone(),
            tail_name: self.tail_name.clone(),
            emtpy_alternative: self.emtpy_alternative.transform_types(transform)?.into(),
            non_emtpy_alternative: self.non_emtpy_alternative.transform_types(transform)?.into(),
            source_information: self.source_information.clone(),
        })
    }
}
