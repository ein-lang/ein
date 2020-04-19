use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::rc::Rc;

pub struct TypeCoercionDesugarer<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
}

impl<'a> TypeCoercionDesugarer<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_type_resolver,
        }
    }

    pub fn desugar(&self, module: &Module) -> Result<Module, CompileError> {
        let mut variables = HashMap::<String, Type>::new();

        for imported_module in module.imported_modules() {
            for (name, type_) in imported_module.variables() {
                variables.insert(imported_module.path().qualify_name(name), type_.clone());
            }
        }

        for type_definition in module.type_definitions() {
            if let Type::Record(record) = type_definition.type_() {
                for (key, type_) in record.elements() {
                    variables.insert(
                        format!("{}.{}", record.name(), key),
                        types::Function::new(
                            record.clone(),
                            type_.clone(),
                            type_.source_information().clone(),
                        )
                        .into(),
                    );
                }
            }
        }

        for definition in module.definitions() {
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

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imported_modules().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .map(|definition| {
                    Ok(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .desugar_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::ValueDefinition(value_definition) => self
                            .desugar_value_definition(value_definition, &variables)?
                            .into(),
                    })
                })
                .collect::<Result<_, CompileError>>()?,
        ))
    }

    fn desugar_function_definition(
        &self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError> {
        let mut variables = variables.clone();
        let mut type_ = function_definition.type_();

        for argument_name in function_definition.arguments() {
            let function_type = type_.to_function().unwrap();

            variables.insert(argument_name.into(), function_type.argument().clone());

            type_ = function_type.result();
        }

        Ok(FunctionDefinition::new(
            function_definition.name(),
            function_definition.arguments().to_vec(),
            self.coerce_type(
                function_definition.body(),
                type_,
                function_definition.source_information().clone(),
                &variables,
            )?,
            function_definition.type_().clone(),
            function_definition.source_information().clone(),
        ))
    }

    fn desugar_value_definition(
        &self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<ValueDefinition, CompileError> {
        Ok(ValueDefinition::new(
            value_definition.name(),
            self.coerce_type(
                value_definition.body(),
                value_definition.type_(),
                value_definition.source_information().clone(),
                &variables,
            )?,
            value_definition.type_().clone(),
            value_definition.source_information().clone(),
        ))
    }

    fn desugar_expression(
        &self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        match expression {
            Expression::Application(application) => {
                let source_information = application.source_information();
                let function_type = self.infer_expression(application.function(), variables);

                Ok(Application::new(
                    self.desugar_expression(application.function(), variables)?,
                    self.coerce_type(
                        application.argument(),
                        function_type.to_function().unwrap().argument(),
                        source_information.clone(),
                        variables,
                    )?,
                    source_information.clone(),
                )
                .into())
            }
            Expression::If(if_) => {
                let result_type = self.infer_expression(expression, variables);

                Ok(If::new(
                    self.desugar_expression(if_.condition(), variables)?,
                    self.coerce_type(
                        if_.then(),
                        &result_type,
                        if_.source_information().clone(),
                        variables,
                    )?,
                    self.coerce_type(
                        if_.else_(),
                        &result_type,
                        if_.source_information().clone(),
                        variables,
                    )?,
                    if_.source_information().clone(),
                )
                .into())
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
                            if let_.has_functions() {
                                variables.insert(
                                    value_definition.name().into(),
                                    value_definition.type_().clone(),
                                );
                            }
                        }
                    }
                }

                let mut definitions = vec![];

                for definition in let_.definitions() {
                    definitions.push(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .desugar_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::ValueDefinition(value_definition) => {
                            let definition =
                                self.desugar_value_definition(value_definition, &variables)?;

                            variables.insert(
                                value_definition.name().into(),
                                value_definition.type_().clone(),
                            );

                            definition.into()
                        }
                    })
                }

                Ok(Let::new(
                    definitions,
                    self.desugar_expression(let_.expression(), &variables)?,
                )
                .into())
            }
            Expression::Operation(_) => {
                // TODO Make operations generic.
                Ok(expression.clone())
            }
            Expression::RecordConstruction(record_construction) => {
                let type_ = self
                    .reference_type_resolver
                    .resolve_reference(record_construction.type_())?;
                let record_type = type_.to_record().unwrap();

                Ok(RecordConstruction::new(
                    record_construction.type_().clone(),
                    record_construction
                        .elements()
                        .iter()
                        .map(|(key, expression)| {
                            Ok((
                                key.clone(),
                                self.coerce_type(
                                    expression,
                                    &record_type.elements()[key],
                                    record_construction.source_information().clone(),
                                    variables,
                                )?,
                            ))
                        })
                        .collect::<Result<_, CompileError>>()?,
                    record_construction.source_information().clone(),
                )
                .into())
            }
            Expression::Boolean(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }

    fn infer_expression(&self, expression: &Expression, variables: &HashMap<String, Type>) -> Type {
        match expression {
            Expression::Application(application) => self
                .infer_expression(application.function(), variables)
                .to_function()
                .unwrap()
                .result()
                .clone(),
            Expression::Boolean(boolean) => {
                types::Boolean::new(boolean.source_information().clone()).into()
            }
            Expression::If(if_) => types::Union::new(
                vec![
                    self.infer_expression(if_.then(), variables),
                    self.infer_expression(if_.else_(), variables),
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

                self.infer_expression(let_.expression(), &variables)
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

    fn coerce_type(
        &self,
        expression: &Expression,
        to_type: &Type,
        source_information: Rc<SourceInformation>,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let from_type = self.infer_expression(expression, variables);
        let expression = self.desugar_expression(expression, variables)?;

        Ok(if &from_type == to_type {
            expression
        } else {
            TypeCoercion::new(expression, from_type, to_type.clone(), source_information).into()
        })
    }
}
