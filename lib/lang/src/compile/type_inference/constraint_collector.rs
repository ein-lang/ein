use super::super::error::CompileError;
use super::super::module_environment_creator::ModuleEnvironmentCreator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption_set::SubsumptionSet;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct ConstraintCollector {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    module_environment_creator: Arc<ModuleEnvironmentCreator>,
    solved_subsumption_set: SubsumptionSet,
    checked_subsumption_set: SubsumptionSet,
}

impl ConstraintCollector {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        module_environment_creator: Arc<ModuleEnvironmentCreator>,
    ) -> Self {
        Self {
            reference_type_resolver,
            module_environment_creator,
            solved_subsumption_set: SubsumptionSet::new(),
            checked_subsumption_set: SubsumptionSet::new(),
        }
    }

    pub fn collect(
        mut self,
        module: &Module,
    ) -> Result<(SubsumptionSet, SubsumptionSet), CompileError> {
        let variables = self.module_environment_creator.create(module);

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    self.infer_function_definition(function_definition, &variables)?;
                }
                Definition::VariableDefinition(variable_definition) => {
                    self.infer_variable_definition(variable_definition, &variables)?;
                }
            };
        }

        Ok((self.solved_subsumption_set, self.checked_subsumption_set))
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

            self.solved_subsumption_set.add(
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
        self.solved_subsumption_set.add(body_type, type_);

        Ok(())
    }

    fn infer_variable_definition(
        &mut self,
        variable_definition: &VariableDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<(), CompileError> {
        let type_ = self.infer_expression(variable_definition.body(), &variables)?;

        self.solved_subsumption_set
            .add(type_, variable_definition.type_().clone());

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

                self.solved_subsumption_set.add(
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

                self.checked_subsumption_set
                    .add(argument.clone(), case.type_().clone());
                self.checked_subsumption_set
                    .add(case.type_().clone(), argument);

                let result = types::Variable::new(case.source_information().clone());

                for alternative in case.alternatives() {
                    self.solved_subsumption_set
                        .add(alternative.type_().clone(), case.type_().clone());

                    let mut variables = variables.clone();

                    variables.insert(case.name().into(), alternative.type_().clone());

                    let type_ = self.infer_expression(alternative.expression(), &variables)?;
                    self.solved_subsumption_set.add(type_, result.clone());
                }

                Ok(result.into())
            }
            Expression::If(if_) => {
                let condition = self.infer_expression(if_.condition(), variables)?;
                self.solved_subsumption_set.add(
                    condition,
                    types::Boolean::new(if_.source_information().clone()),
                );

                let then = self.infer_expression(if_.then(), variables)?;
                let else_ = self.infer_expression(if_.else_(), variables)?;
                let result = types::Variable::new(if_.source_information().clone());

                self.solved_subsumption_set.add(then, result.clone());
                self.solved_subsumption_set.add(else_, result.clone());

                Ok(result.into())
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                for definition in let_.definitions() {
                    if let Definition::FunctionDefinition(function_definition) = definition {
                        variables.insert(
                            function_definition.name().into(),
                            function_definition.type_().clone(),
                        );
                    }
                }

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            self.infer_function_definition(function_definition, &variables)?;
                        }
                        Definition::VariableDefinition(variable_definition) => {
                            self.infer_variable_definition(variable_definition, &variables)?;

                            variables.insert(
                                variable_definition.name().into(),
                                variable_definition.type_().clone(),
                            );
                        }
                    }
                }

                self.infer_expression(let_.expression(), &variables)
            }
            Expression::List(list) => {
                let element_type = types::Variable::new(list.source_information().clone());
                let list_type =
                    types::List::new(element_type.clone(), list.source_information().clone());

                self.solved_subsumption_set
                    .add(list_type.clone(), list.type_().clone());

                for element in list.elements() {
                    match element {
                        ListElement::Multiple(expression) => {
                            let type_ = self.infer_expression(expression, variables)?;
                            self.solved_subsumption_set.add(type_, list_type.clone())
                        }
                        ListElement::Single(expression) => {
                            let type_ = self.infer_expression(expression, variables)?;
                            self.solved_subsumption_set.add(type_, element_type.clone())
                        }
                    }
                }

                Ok(list_type.into())
            }
            Expression::ListCase(case) => {
                let element = types::Variable::new(case.source_information().clone());
                let list = types::List::new(element.clone(), case.source_information().clone());
                let result = types::Variable::new(case.source_information().clone());

                self.solved_subsumption_set
                    .add(list.clone(), case.type_().clone());

                let argument = self.infer_expression(case.argument(), variables)?;
                self.solved_subsumption_set.add(argument, list.clone());

                let type_ = self.infer_expression(case.empty_alternative(), &variables)?;
                self.solved_subsumption_set.add(type_, result.clone());

                let mut variables = variables.clone();
                variables.insert(case.first_name().into(), element.into());
                variables.insert(case.rest_name().into(), list.into());

                let type_ = self.infer_expression(case.non_empty_alternative(), &variables)?;
                self.solved_subsumption_set.add(type_, result.clone());

                Ok(result.into())
            }
            Expression::None(none) => {
                Ok(types::None::new(none.source_information().clone()).into())
            }
            Expression::Number(number) => {
                Ok(types::Number::new(number.source_information().clone()).into())
            }
            Expression::Operation(operation) => match operation {
                Operation::Boolean(operation) => {
                    let lhs = self.infer_expression(operation.lhs(), variables)?;
                    let rhs = self.infer_expression(operation.rhs(), variables)?;
                    let boolean_type = types::Boolean::new(operation.source_information().clone());

                    self.solved_subsumption_set.add(lhs, boolean_type.clone());
                    self.solved_subsumption_set.add(rhs, boolean_type.clone());

                    Ok(boolean_type.into())
                }
                Operation::Generic(operation) => {
                    let lhs = self.infer_expression(operation.lhs(), variables)?;
                    let rhs = self.infer_expression(operation.rhs(), variables)?;

                    self.solved_subsumption_set
                        .add(lhs.clone(), operation.type_().clone());
                    self.solved_subsumption_set
                        .add(rhs.clone(), operation.type_().clone());

                    Ok(match operation.operator() {
                        Operator::Add
                        | Operator::Subtract
                        | Operator::Multiply
                        | Operator::Divide
                        | Operator::LessThan
                        | Operator::LessThanOrEqual
                        | Operator::GreaterThan
                        | Operator::GreaterThanOrEqual => {
                            let number_type =
                                types::Number::new(operation.source_information().clone());

                            self.solved_subsumption_set.add(lhs, number_type.clone());
                            self.solved_subsumption_set.add(rhs, number_type.clone());

                            match operation.operator() {
                                Operator::Add
                                | Operator::Subtract
                                | Operator::Multiply
                                | Operator::Divide => number_type.into(),
                                _ => types::Boolean::new(operation.source_information().clone())
                                    .into(),
                            }
                        }
                        Operator::Equal | Operator::NotEqual => {
                            types::Boolean::new(operation.source_information().clone()).into()
                        }
                    })
                }
            },
            Expression::RecordConstruction(construction) => {
                let record_type = self
                    .reference_type_resolver
                    .resolve_to_record(construction.type_())?
                    .ok_or_else(|| {
                        CompileError::TypesNotMatched(
                            construction.source_information().clone(),
                            construction.type_().source_information().clone(),
                        )
                    })?;

                if construction.elements().keys().collect::<HashSet<_>>()
                    != record_type.elements().keys().collect()
                {
                    return Err(CompileError::TypesNotMatched(
                        construction.source_information().clone(),
                        record_type.source_information().clone(),
                    ));
                }

                for (key, expression) in construction.elements() {
                    let type_ = self.infer_expression(expression, variables)?;

                    self.solved_subsumption_set
                        .add(type_, record_type.elements()[key].clone());
                }

                Ok(construction.type_().clone())
            }
            Expression::RecordElementOperation(operation) => {
                let record_type = self
                    .reference_type_resolver
                    .resolve_to_record(operation.type_())?
                    .ok_or_else(|| {
                        CompileError::TypesNotMatched(
                            operation.source_information().clone(),
                            operation.type_().source_information().clone(),
                        )
                    })?;

                let argument = self.infer_expression(operation.argument(), variables)?;
                self.solved_subsumption_set
                    .add(argument, operation.type_().clone());

                let mut variables = variables.clone();

                variables.insert(
                    operation.variable().into(),
                    record_type.elements()[operation.key()].clone(),
                );

                self.infer_expression(operation.expression(), &variables)
            }
            Expression::String(string) => {
                Ok(types::EinString::new(string.source_information().clone()).into())
            }
            Expression::Variable(variable) => variables
                .get(variable.name())
                .cloned()
                .ok_or_else(|| CompileError::VariableNotFound(variable.clone())),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }
}
