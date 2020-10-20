use super::expression::Expression;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct RecordElementOperation {
    type_: Type,
    key: String,
    argument: Arc<Expression>,
    variable: String,
    expression: Arc<Expression>,
    source_information: Arc<SourceInformation>,
}

impl RecordElementOperation {
    pub fn new(
        type_: impl Into<Type>,
        key: impl Into<String>,
        argument: impl Into<Expression>,
        variable: impl Into<String>,
        expression: impl Into<Expression>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            type_: type_.into(),
            key: key.into(),
            argument: Arc::new(argument.into()),
            variable: variable.into(),
            expression: Arc::new(expression.into()),
            source_information: source_information.into(),
        }
    }

    pub fn type_(&self) -> &Type {
        &self.type_
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn argument(&self) -> &Expression {
        &self.argument
    }

    pub fn variable(&self) -> &str {
        &self.variable
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn convert_expressions<E>(
        &self,
        convert: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.clone(),
            &self.key,
            self.argument.convert_expressions(convert)?,
            &self.variable,
            self.expression.convert_expressions(convert)?,
            self.source_information.clone(),
        ))
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.type_.convert_types(convert)?,
            &self.key,
            self.argument.convert_types(convert)?,
            &self.variable,
            self.expression.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}
