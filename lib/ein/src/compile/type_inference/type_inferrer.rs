use super::super::error::CompileError;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::subsumption::Subsumption;
use super::subsumption_set::SubsumptionSet;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct TypeInferrer<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
    subsumption_set: SubsumptionSet,
}

impl<'a> TypeInferrer<'a> {
    pub fn new(reference_type_resolver: &'a ReferenceTypeResolver) -> Self {
        Self {
            reference_type_resolver,
            subsumption_set: SubsumptionSet::new(),
        }
    }

    pub fn infer(&mut self, module: &Module) -> Result<Module, CompileError> {
        self.collect_subsumptions(module)?;

        let substitutions = self.reduce_subsumptions()?;

        Ok(module.convert_types(&mut |type_| {
            if let Type::Variable(variable) = type_ {
                substitutions[&variable.id()].clone()
            } else {
                type_.clone()
            }
        }))
    }

    fn collect_subsumptions(&mut self, module: &Module) -> Result<(), CompileError> {
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

        Ok(())
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

            self.subsumption_set.add_subsumption(
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
        self.subsumption_set.add_subsumption(body_type, type_);

        Ok(())
    }

    fn infer_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<(), CompileError> {
        let type_ = self.infer_expression(value_definition.body(), &variables)?;

        self.subsumption_set
            .add_subsumption(type_, value_definition.type_().clone());

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

                self.subsumption_set.add_subsumption(
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
            Expression::If(if_) => {
                let condition = self.infer_expression(if_.condition(), variables)?;
                self.subsumption_set.add_subsumption(
                    condition,
                    types::Boolean::new(if_.source_information().clone()),
                );

                let then = self.infer_expression(if_.then(), variables)?;
                let else_ = self.infer_expression(if_.else_(), variables)?;

                Ok(types::Union::new(vec![then, else_], if_.source_information().clone()).into())
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                // Because the language does not have let-rec
                // expression like OCaml, we need to guess if the
                // let expression is recursive or not to generate
                // proper type subsumptions.
                let functions_included =
                    let_.definitions()
                        .iter()
                        .any(|definition| match definition {
                            Definition::FunctionDefinition(_) => true,
                            Definition::ValueDefinition(value_definition) => {
                                if let Type::Function(_) = value_definition.type_() {
                                    true
                                } else {
                                    false
                                }
                            }
                        });

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            variables.insert(
                                function_definition.name().into(),
                                function_definition.type_().clone(),
                            );
                        }
                        Definition::ValueDefinition(value_definition) => {
                            if functions_included {
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
                self.subsumption_set
                    .add_subsumption(lhs, number_type.clone());
                let rhs = self.infer_expression(operation.rhs(), variables)?;
                self.subsumption_set
                    .add_subsumption(rhs, number_type.clone());

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
                        .add_subsumption(type_, record_type.elements()[key].clone());
                }

                Ok(record.type_().clone().into())
            }
            Expression::RecordUpdate(_) => unreachable!(),
            Expression::Variable(variable) => variables
                .get(variable.name())
                .cloned()
                .ok_or_else(|| CompileError::VariableNotFound(variable.clone())),
        }
    }

    fn reduce_subsumptions(&mut self) -> Result<HashMap<usize, Type>, CompileError> {
        let mut substitutions = HashMap::<usize, Type>::new();

        while let Some(subsumption) = self.subsumption_set.remove() {
            match (subsumption.lower(), subsumption.upper()) {
                (Type::Variable(variable), upper) => {
                    if let Type::Variable(other_variable) = upper {
                        if variable.id() == other_variable.id() {
                            continue;
                        }
                    }

                    self.substitute_variable(variable, upper, &mut substitutions);
                }
                (lower, Type::Variable(variable)) => {
                    self.substitute_variable(variable, lower, &mut substitutions);
                }
                (Type::Reference(reference), other) => self.subsumption_set.add_subsumption(
                    self.reference_type_resolver.resolve_reference(reference)?,
                    other.clone(),
                ),
                (one, Type::Reference(reference)) => self.subsumption_set.add_subsumption(
                    one.clone(),
                    self.reference_type_resolver.resolve_reference(reference)?,
                ),
                (Type::Function(one), Type::Function(other)) => {
                    self.subsumption_set
                        .add_subsumption(one.argument().clone(), other.argument().clone());
                    self.subsumption_set
                        .add_subsumption(other.result().clone(), one.result().clone());
                }
                (Type::Union(union), other) => {
                    for type_ in union.types() {
                        self.subsumption_set
                            .add_subsumption(type_.clone(), other.clone());
                    }
                }
                (_, Type::Union(_)) => unreachable!(),
                (Type::Boolean(_), Type::Boolean(_)) => {}
                (Type::None(_), Type::None(_)) => {}
                (Type::Number(_), Type::Number(_)) => {}
                (Type::Record(one), Type::Record(other)) => {
                    if one.name() != other.name() {
                        return Err(CompileError::TypesNotMatched(
                            one.source_information().clone(),
                            other.source_information().clone(),
                        ));
                    };
                }
                (lower, upper) => {
                    return Err(CompileError::TypesNotMatched(
                        lower.source_information().clone(),
                        upper.source_information().clone(),
                    ))
                }
            }
        }

        Ok(substitutions)
    }

    fn substitute_variable(
        &mut self,
        variable: &types::Variable,
        type_: &Type,
        substitutions: &mut HashMap<usize, Type>,
    ) {
        for (_, substituted_type) in substitutions.iter_mut() {
            *substituted_type = substituted_type
                .clone()
                .substitute_variable(variable, type_);
        }

        for subsumption in self.subsumption_set.iter_mut() {
            *subsumption = Subsumption::new(
                subsumption.lower().substitute_variable(variable, type_),
                subsumption.upper().substitute_variable(variable, type_),
            )
        }

        substitutions.insert(variable.id(), type_.clone());
    }
}
