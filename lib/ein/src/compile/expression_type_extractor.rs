use super::error::CompileError;
use super::union_type_simplifier::UnionTypeSimplifier;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct ExpressionTypeExtractor<'a> {
    union_type_simplifier: &'a UnionTypeSimplifier<'a>,
}

impl<'a> ExpressionTypeExtractor<'a> {
    pub fn new(union_type_simplifier: &'a UnionTypeSimplifier<'a>) -> Self {
        Self {
            union_type_simplifier,
        }
    }

    pub fn extract(
        &self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Type, CompileError> {
        Ok(match expression {
            Expression::Application(application) => self
                .extract(application.function(), variables)?
                .to_function()
                .unwrap()
                .result()
                .clone(),
            Expression::Boolean(boolean) => {
                types::Boolean::new(boolean.source_information().clone()).into()
            }
            Expression::If(if_) => {
                self.union_type_simplifier
                    .simplify_union(&types::Union::new(
                        vec![
                            self.extract(if_.then(), variables)?,
                            self.extract(if_.else_(), variables)?,
                        ],
                        if_.source_information().clone(),
                    ))?
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            variables.insert(
                                function_definition.name().into(),
                                function_definition.type_().clone(),
                            );
                        }
                        Definition::ValueDefinition(value_definition) => {
                            variables.insert(
                                value_definition.name().into(),
                                value_definition.type_().clone(),
                            );
                        }
                    }
                }

                self.extract(let_.expression(), &variables)?
            }
            Expression::None(none) => types::None::new(none.source_information().clone()).into(),
            Expression::Number(number) => {
                types::Number::new(number.source_information().clone()).into()
            }
            Expression::Operation(operation) => match operation.operator() {
                Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
                    types::Number::new(operation.source_information().clone()).into()
                }
                Operator::Equal
                | Operator::NotEqual
                | Operator::LessThan
                | Operator::LessThanOrEqual
                | Operator::GreaterThan
                | Operator::GreaterThanOrEqual => {
                    types::Boolean::new(operation.source_information().clone()).into()
                }
            },
            Expression::RecordConstruction(record) => record.type_().clone().into(),
            Expression::TypeCoercion(coercion) => coercion.to().clone(),
            Expression::Variable(variable) => variables[variable.name()].clone(),
            Expression::RecordUpdate(_) => unreachable!(),
        })
    }
}
