use super::equation::*;
use super::equation_set::EquationSet;
use super::error::*;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::*;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct TypeInferrer {
    environment: HashMap<String, Type>,
    equation_set: EquationSet,
}

impl TypeInferrer {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            equation_set: EquationSet::new(),
        }
    }

    pub fn infer(&mut self, module: &Module) -> Result<Module, TypeInferenceError> {
        for imported_module in module.imported_modules() {
            self.environment.extend(
                imported_module.types().iter().map(|(name, type_)| {
                    (imported_module.path().qualify_name(name), type_.clone())
                }),
            );
        }

        for type_definition in module.type_definitions() {
            self.environment.insert(
                type_definition.name().into(),
                type_definition.type_().clone(),
            );
        }

        self.collect_equations(module)?;

        let substitutions = self.reduce_equations()?;

        Ok(module.convert_types(&mut |type_| {
            if let Type::Variable(variable) = type_ {
                substitutions[&variable.id()].clone()
            } else {
                type_.clone()
            }
        }))
    }

    fn collect_equations(&mut self, module: &Module) -> Result<(), TypeInferenceError> {
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
    ) -> Result<(), TypeInferenceError> {
        let source_information = function_definition.source_information();
        let mut variables = variables.clone();
        let mut type_ = function_definition.type_().clone();

        for argument_name in function_definition.arguments() {
            let argument_type: Type = types::Variable::new(source_information.clone()).into();
            let result_type: Type = types::Variable::new(source_information.clone()).into();

            self.equation_set.add(Equation::new(
                type_,
                types::Function::new(
                    argument_type.clone(),
                    result_type.clone(),
                    source_information.clone(),
                ),
            ));

            variables.insert(argument_name.into(), argument_type);

            type_ = result_type;
        }

        let body_type = self.infer_expression(function_definition.body(), &variables)?;
        self.equation_set.add(Equation::new(body_type, type_));

        Ok(())
    }

    fn infer_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<(), TypeInferenceError> {
        let type_ = self.infer_expression(value_definition.body(), &variables)?;

        self.equation_set
            .add(Equation::new(type_, value_definition.type_().clone()));

        Ok(())
    }

    fn infer_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Type, TypeInferenceError> {
        match expression {
            Expression::Application(application) => {
                let function = self.infer_expression(application.function(), variables)?;
                let argument = self.infer_expression(application.argument(), variables)?;
                let result: Type =
                    types::Variable::new(application.source_information().clone()).into();

                self.equation_set.add(Equation::new(
                    function,
                    types::Function::new(
                        argument,
                        result.clone(),
                        application.source_information().clone(),
                    ),
                ));

                Ok(result)
            }
            Expression::Boolean(boolean) => {
                Ok(types::Boolean::new(boolean.source_information().clone()).into())
            }
            Expression::If(if_) => {
                let condition = self.infer_expression(if_.condition(), variables)?;
                self.equation_set.add(Equation::new(
                    condition,
                    types::Boolean::new(if_.source_information().clone()),
                ));

                let then = self.infer_expression(if_.then(), variables)?;
                let else_ = self.infer_expression(if_.else_(), variables)?;

                self.equation_set.add(Equation::new(then.clone(), else_));

                Ok(then)
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                // Because the language does not have let-rec
                // expression like OCaml, we need to guess if the
                // let expression is recursive or not to generate
                // proper type equations.
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
                let type_: Type = types::Number::new(operation.source_information().clone()).into();

                let lhs = self.infer_expression(operation.lhs(), variables)?;
                self.equation_set.add(Equation::new(lhs, type_.clone()));
                let rhs = self.infer_expression(operation.rhs(), variables)?;
                self.equation_set.add(Equation::new(rhs, type_.clone()));

                Ok(type_)
            }
            Expression::RecordConstruction(record) => {
                let record_type = self
                    .environment
                    .get(record.type_().name())
                    .and_then(|type_| type_.to_record())
                    .ok_or_else(|| TypeInferenceError::TypeNotFound {
                        reference: record.type_().clone(),
                    })?
                    .clone();

                if HashSet::<&String>::from_iter(record.elements().keys())
                    != HashSet::from_iter(record_type.elements().keys())
                {
                    return Err(TypeInferenceError::TypesNotMatched(
                        record.source_information().clone(),
                        record_type.source_information().clone(),
                    ));
                }

                for (key, expression) in record.elements() {
                    let type_ = self.infer_expression(expression, variables)?;

                    self.equation_set
                        .add(Equation::new(type_, record_type.elements()[key].clone()));
                }

                Ok(record.type_().clone().into())
            }
            Expression::RecordUpdate(_) => unreachable!(),
            Expression::Variable(variable) => {
                variables.get(variable.name()).cloned().ok_or_else(|| {
                    TypeInferenceError::VariableNotFound(
                        variable.name().into(),
                        variable.source_information().clone(),
                    )
                })
            }
        }
    }

    fn reduce_equations(&mut self) -> Result<HashMap<usize, Type>, TypeInferenceError> {
        let mut substitutions = HashMap::<usize, Type>::new();

        while let Some(equation) = self.equation_set.remove() {
            match (equation.lhs(), equation.rhs()) {
                (Type::Variable(variable), rhs) => {
                    if let Type::Variable(other_variable) = rhs {
                        if variable.id() == other_variable.id() {
                            continue;
                        }
                    }

                    self.substitute_variable(variable, rhs, &mut substitutions);
                }
                (lhs, Type::Variable(variable)) => {
                    self.substitute_variable(variable, lhs, &mut substitutions);
                }
                (Type::Reference(reference), other) => self.equation_set.add(Equation::new(
                    self.environment
                        .get(reference.name())
                        .ok_or_else(|| TypeInferenceError::TypeNotFound {
                            reference: reference.clone(),
                        })?
                        .clone(),
                    other.clone(),
                )),
                (one, Type::Reference(reference)) => self.equation_set.add(Equation::new(
                    one.clone(),
                    self.environment
                        .get(reference.name())
                        .ok_or_else(|| TypeInferenceError::TypeNotFound {
                            reference: reference.clone(),
                        })?
                        .clone(),
                )),
                (Type::Function(one), Type::Function(other)) => {
                    self.equation_set.add(Equation::new(
                        one.argument().clone(),
                        other.argument().clone(),
                    ));

                    self.equation_set
                        .add(Equation::new(other.result().clone(), one.result().clone()));
                }
                (Type::Boolean(_), Type::Boolean(_)) => {}
                (Type::None(_), Type::None(_)) => {}
                (Type::Number(_), Type::Number(_)) => {}
                (Type::Record(one), Type::Record(other)) => {
                    if one.name() != other.name() {
                        return Err(TypeInferenceError::TypesNotMatched(
                            one.source_information().clone(),
                            other.source_information().clone(),
                        ));
                    };
                }
                (lhs, rhs) => {
                    return Err(TypeInferenceError::TypesNotMatched(
                        lhs.source_information().clone(),
                        rhs.source_information().clone(),
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

        for equation in self.equation_set.iter_mut() {
            *equation = Equation::new(
                equation.lhs().substitute_variable(variable, type_),
                equation.rhs().substitute_variable(variable, type_),
            )
        }

        substitutions.insert(variable.id(), type_.clone());
    }
}
