use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct ExpressionTypeExtractor {}

impl<'a> ExpressionTypeExtractor {
    pub fn extract(expression: &Expression, variables: &HashMap<String, Type>) -> Type {
        match expression {
            Expression::Application(application) => {
                Self::extract(application.function(), variables)
                    .to_function()
                    .unwrap()
                    .result()
                    .clone()
            }
            Expression::Boolean(boolean) => {
                types::Boolean::new(boolean.source_information().clone()).into()
            }
            Expression::If(if_) => types::Union::new(
                vec![
                    Self::extract(if_.then(), variables),
                    Self::extract(if_.else_(), variables),
                ],
                if_.source_information().clone(),
            )
            .simplify(),
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

                Self::extract(let_.expression(), &variables)
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
        }
    }
}
