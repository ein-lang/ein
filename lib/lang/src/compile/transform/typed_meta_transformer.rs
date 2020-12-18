use super::super::error::CompileError;
use super::super::module_environment_creator::ModuleEnvironmentCreator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

pub trait TypedTransformer {
    fn transform_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError>;
    fn transform_variable_definition(
        &mut self,
        variable_definition: &VariableDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<VariableDefinition, CompileError>;
    fn transform_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError>;
}

pub struct TypedMetaTransformer<D> {
    component_transformer: D,
    module_environment_creator: Arc<ModuleEnvironmentCreator>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl<D: TypedTransformer> TypedMetaTransformer<D> {
    pub fn new(
        component_transformer: D,
        module_environment_creator: Arc<ModuleEnvironmentCreator>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
    ) -> Self {
        Self {
            component_transformer,
            module_environment_creator,
            reference_type_resolver,
        }
    }

    pub fn transform(&mut self, module: &Module) -> Result<Module, CompileError> {
        let variables = self.module_environment_creator.create(module);

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imports().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .map(|definition| {
                    Ok(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .transform_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::VariableDefinition(variable_definition) => self
                            .transform_variable_definition(variable_definition, &variables)?
                            .into(),
                    })
                })
                .collect::<Result<_, CompileError>>()?,
        ))
    }

    fn transform_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError> {
        let mut variables = variables.clone();

        for (name, type_) in function_definition.arguments().iter().zip(
            self.reference_type_resolver
                .resolve_to_function(function_definition.type_())?
                .unwrap()
                .arguments(),
        ) {
            variables.insert(name.into(), type_.clone());
        }

        let body = self.transform_expression(function_definition.body(), &variables)?;

        Ok(self.component_transformer.transform_function_definition(
            &FunctionDefinition::new(
                function_definition.name(),
                function_definition.arguments().to_vec(),
                body,
                function_definition.type_().clone(),
                function_definition.source_information().clone(),
            ),
            &variables,
        )?)
    }

    fn transform_variable_definition(
        &mut self,
        variable_definition: &VariableDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<VariableDefinition, CompileError> {
        let body = self.transform_expression(variable_definition.body(), variables)?;

        Ok(self.component_transformer.transform_variable_definition(
            &VariableDefinition::new(
                variable_definition.name(),
                body,
                variable_definition.type_().clone(),
                variable_definition.source_information().clone(),
            ),
            variables,
        )?)
    }

    fn transform_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let expression = match expression {
            Expression::Application(application) => Application::new(
                self.transform_expression(application.function(), variables)?,
                self.transform_expression(application.argument(), variables)?,
                application.source_information().clone(),
            )
            .into(),
            Expression::Case(case) => Case::with_type(
                case.type_().clone(),
                case.name(),
                self.transform_expression(case.argument(), variables)?,
                case.alternatives()
                    .iter()
                    .map(|alternative| {
                        let mut variables = variables.clone();

                        variables.insert(case.name().into(), alternative.type_().clone());

                        Ok(Alternative::new(
                            alternative.type_().clone(),
                            self.transform_expression(alternative.expression(), &variables)?,
                        ))
                    })
                    .collect::<Result<_, CompileError>>()?,
                case.source_information().clone(),
            )
            .into(),
            Expression::If(if_) => If::new(
                self.transform_expression(if_.condition(), variables)?,
                self.transform_expression(if_.then(), variables)?,
                self.transform_expression(if_.else_(), variables)?,
                if_.source_information().clone(),
            )
            .into(),
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
                        Definition::VariableDefinition(_) => {}
                    }
                }

                let mut definitions = vec![];

                for definition in let_.definitions() {
                    definitions.push(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .transform_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::VariableDefinition(variable_definition) => {
                            let definition = self
                                .transform_variable_definition(variable_definition, &variables)?;

                            variables.insert(
                                variable_definition.name().into(),
                                variable_definition.type_().clone(),
                            );

                            definition.into()
                        }
                    })
                }

                Let::new(
                    definitions,
                    self.transform_expression(let_.expression(), &variables)?,
                    let_.source_information().clone(),
                )
                .into()
            }
            Expression::List(list) => List::with_type(
                list.type_().clone(),
                list.elements()
                    .iter()
                    .map(|element| match element {
                        ListElement::Multiple(expression) => Ok(ListElement::Multiple(
                            self.transform_expression(expression, &variables)?,
                        )),
                        ListElement::Single(expression) => Ok(ListElement::Single(
                            self.transform_expression(expression, &variables)?,
                        )),
                    })
                    .collect::<Result<Vec<_>, CompileError>>()?,
                list.source_information().clone(),
            )
            .into(),
            Expression::ListCase(case) => ListCase::new(
                self.transform_expression(case.argument(), variables)?,
                case.type_().clone(),
                case.first_name(),
                case.rest_name(),
                self.transform_expression(case.empty_alternative(), &variables)?,
                {
                    let mut variables = variables.clone();

                    variables.insert(
                        case.first_name().into(),
                        self.reference_type_resolver
                            .resolve_to_list(case.type_())?
                            .unwrap()
                            .element()
                            .clone(),
                    );
                    variables.insert(case.rest_name().into(), case.type_().clone());

                    self.transform_expression(case.non_empty_alternative(), &variables)?
                },
                case.source_information().clone(),
            )
            .into(),
            Expression::Operation(operation) => Operation::with_type(
                operation.type_().clone(),
                operation.operator(),
                self.transform_expression(operation.lhs(), &variables)?,
                self.transform_expression(operation.rhs(), &variables)?,
                operation.source_information().clone(),
            )
            .into(),
            Expression::RecordConstruction(record_construction) => RecordConstruction::new(
                record_construction.type_().clone(),
                record_construction
                    .elements()
                    .iter()
                    .map(|(key, expression)| {
                        Ok((
                            key.clone(),
                            self.transform_expression(expression, variables)?,
                        ))
                    })
                    .collect::<Result<_, CompileError>>()?,
                record_construction.source_information().clone(),
            )
            .into(),
            Expression::RecordElementOperation(operation) => RecordElementOperation::new(
                operation.type_().clone(),
                operation.key(),
                self.transform_expression(operation.argument(), variables)?,
                operation.variable(),
                {
                    let mut variables = variables.clone();

                    variables.insert(
                        operation.variable().into(),
                        self.reference_type_resolver
                            .resolve_to_record(operation.type_())?
                            .unwrap()
                            .elements()[operation.key()]
                        .clone(),
                    );

                    self.transform_expression(operation.expression(), &variables)?
                },
                operation.source_information().clone(),
            )
            .into(),
            Expression::Boolean(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::String(_)
            | Expression::Variable(_) => expression.clone(),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        };

        Ok(self
            .component_transformer
            .transform_expression(&expression, &variables)?)
    }
}
