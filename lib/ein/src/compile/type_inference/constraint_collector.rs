use super::super::error::CompileError;
use super::super::module_environment_creator::ModuleEnvironmentCreator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption_set::SubsumptionSet;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct ConstraintCollector<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
    subsumption_set: SubsumptionSet,
}

impl<'a> ConstraintCollector<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_type_resolver,
            subsumption_set: SubsumptionSet::new(),
        }
    }

    pub fn collect(mut self, module: &Module) -> Result<SubsumptionSet, CompileError> {
        let variables = ModuleEnvironmentCreator::create(module);

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    self.infer_function_definition(function_definition, &variables)?;
                }
                Definition::ValueDefinition(value_definition) => {
                    self.infer_value_definition(value_definition, &variables)?;
                }
            };
        }

        Ok(self.subsumption_set)
    }

    fn infer_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<(), CompileError> {
        let source_information = function_definition.source_information();
        let mut variables = variables.clone();
        let mut type_ = function_definition.type_().clone();

        for argument_name in function_definition.arguments() {
            let argument_type: Type = types::Variable::new(source_information.clone()).into();
            let result_type: Type = types::Variable::new(source_information.clone()).into();

            self.subsumption_set.add(
                types::Function::new(
                    argument_type.clone(),
                    result_type.clone(),
                    source_information.clone(),
                ),
                type_,
            );

            variables.insert(argument_name.into(), argument_type);

            type_ = result_type;
        }

        let body_type = self.infer_expression(function_definition.body(), &variables)?;
        self.subsumption_set.add(body_type, type_);

        Ok(())
    }

    fn infer_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<(), CompileError> {
        let type_ = self.infer_expression(value_definition.body(), &variables)?;

        self.subsumption_set
            .add(type_, value_definition.type_().clone());

        Ok(())
    }

    fn infer_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Type, CompileError> {
        match expression {
            Expression::Application(application) => {
                let function = self.infer_expression(application.function(), variables)?;
                let argument = self.infer_expression(application.argument(), variables)?;
                let result: Type =
                    types::Variable::new(application.source_information().clone()).into();

                self.subsumption_set.add(
                    function,
                    types::Function::new(
                        argument,
                        result.clone(),
                        application.source_information().clone(),
                    ),
                );

                Ok(result)
            }
            Expression::Boolean(boolean) => {
                Ok(types::Boolean::new(boolean.source_information().clone()).into())
            }
            Expression::Case(case) => {
                let argument = self.infer_expression(case.argument(), variables)?;

                self.subsumption_set
                    .add(argument.clone(), case.type_().clone());

                let result = types::Variable::new(case.source_information().clone());

                for alternative in case.alternatives() {
                    self.subsumption_set
                        .add(alternative.type_().clone(), argument.clone());

                    let mut variables = variables.clone();

                    variables.insert(alternative.name().into(), alternative.type_().clone());

                    let type_ = self.infer_expression(alternative.expression(), &variables)?;
                    self.subsumption_set.add(type_, result.clone());
                }

                Ok(result.into())
            }
            Expression::If(if_) => {
                let condition = self.infer_expression(if_.condition(), variables)?;
                self.subsumption_set.add(
                    condition,
                    types::Boolean::new(if_.source_information().clone()),
                );

                let then = self.infer_expression(if_.then(), variables)?;
                let else_ = self.infer_expression(if_.else_(), variables)?;
                let result = types::Variable::new(if_.source_information().clone());

                self.subsumption_set.add(then, result.clone());
                self.subsumption_set.add(else_, result.clone());

                Ok(result.into())
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

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            self.infer_function_definition(function_definition, &variables)?;
                        }
                        Definition::ValueDefinition(value_definition) => {
                            self.infer_value_definition(value_definition, &variables)?;

                            variables.insert(
                                value_definition.name().into(),
                                value_definition.type_().clone(),
                            );
                        }
                    }
                }

                self.infer_expression(let_.expression(), &variables)
            }
            Expression::None(none) => {
                Ok(types::None::new(none.source_information().clone()).into())
            }
            Expression::Number(number) => {
                Ok(types::Number::new(number.source_information().clone()).into())
            }
            Expression::Operation(operation) => {
                let number_type = types::Number::new(operation.source_information().clone());

                let lhs = self.infer_expression(operation.lhs(), variables)?;
                self.subsumption_set.add(lhs, number_type.clone());
                let rhs = self.infer_expression(operation.rhs(), variables)?;
                self.subsumption_set.add(rhs, number_type.clone());

                Ok(match operation.operator() {
                    Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
                        number_type.into()
                    }
                    Operator::Equal
                    | Operator::NotEqual
                    | Operator::LessThan
                    | Operator::LessThanOrEqual
                    | Operator::GreaterThan
                    | Operator::GreaterThanOrEqual => {
                        types::Boolean::new(number_type.source_information().clone()).into()
                    }
                })
            }
            Expression::RecordConstruction(record) => {
                let type_ = self
                    .reference_type_resolver
                    .resolve_reference(record.type_())?;
                let record_type = type_.to_record().ok_or_else(|| {
                    CompileError::TypesNotMatched(
                        record.source_information().clone(),
                        type_.source_information().clone(),
                    )
                })?;

                if HashSet::<&String>::from_iter(record.elements().keys())
                    != HashSet::from_iter(record_type.elements().keys())
                {
                    return Err(CompileError::TypesNotMatched(
                        record.source_information().clone(),
                        record_type.source_information().clone(),
                    ));
                }

                for (key, expression) in record.elements() {
                    let type_ = self.infer_expression(expression, variables)?;

                    self.subsumption_set
                        .add(type_, record_type.elements()[key].clone());
                }

                Ok(record.type_().clone().into())
            }
            Expression::Variable(variable) => variables
                .get(variable.name())
                .cloned()
                .ok_or_else(|| CompileError::VariableNotFound(variable.clone())),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }
}
