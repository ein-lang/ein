use super::equation::*;
use super::error::*;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::*;

#[derive(Clone, Debug)]
pub struct TypeInferer {
    equations: Vec<Equation>,
}

impl TypeInferer {
    pub fn new() -> Self {
        Self { equations: vec![] }
    }

    pub fn infer(&mut self, module: &Module) -> Result<Module, TypeInferenceError> {
        self.collect_equations(module)?;
        Ok(module.substitute_type_variables(&self.reduce_equations()?))
    }

    fn collect_equations(&mut self, module: &Module) -> Result<(), TypeInferenceError> {
        let mut variables = HashMap::<&str, Type>::new();

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    variables.insert(
                        function_definition.name(),
                        function_definition.type_().clone().into(),
                    );
                }
                Definition::ValueDefinition(value_definition) => {
                    variables.insert(value_definition.name(), value_definition.type_().clone());
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
        variables: &HashMap<&str, Type>,
    ) -> Result<(), TypeInferenceError> {
        let source_information = function_definition.source_information();
        let mut variables = variables.clone();
        let mut type_ = function_definition.type_().clone();

        for argument_name in function_definition.arguments() {
            let argument_type: Type = types::Variable::new(source_information.clone()).into();
            let result_type: Type = types::Variable::new(source_information.clone()).into();

            self.equations.push(Equation::new(
                type_,
                types::Function::new(
                    argument_type.clone(),
                    result_type.clone(),
                    source_information.clone(),
                )
                .into(),
            ));

            variables.insert(argument_name, argument_type);

            type_ = result_type;
        }

        let body_type = self.infer_expression(function_definition.body(), &variables)?;
        self.equations.push(Equation::new(body_type, type_));

        Ok(())
    }

    fn infer_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<&str, Type>,
    ) -> Result<(), TypeInferenceError> {
        let type_ = self.infer_expression(value_definition.body(), &variables)?;

        self.equations
            .push(Equation::new(type_, value_definition.type_().clone()));

        Ok(())
    }

    fn infer_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<&str, Type>,
    ) -> Result<Type, TypeInferenceError> {
        match expression {
            Expression::Application(application) => {
                let function = self.infer_expression(application.function(), variables)?;
                let argument = self.infer_expression(application.argument(), variables)?;
                let result: Type =
                    types::Variable::new(application.source_information().clone()).into();

                self.equations.push(Equation::new(
                    function,
                    types::Function::new(
                        argument,
                        result.clone(),
                        application.source_information().clone(),
                    )
                    .into(),
                ));

                Ok(result)
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            variables.insert(
                                function_definition.name(),
                                function_definition.type_().clone().into(),
                            );
                        }
                        Definition::ValueDefinition(_) => {}
                    }
                }

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            self.infer_function_definition(function_definition, &variables)?;
                        }
                        Definition::ValueDefinition(value_definition) => {
                            self.infer_value_definition(value_definition, &variables)?;

                            variables
                                .insert(value_definition.name(), value_definition.type_().clone());
                        }
                    }
                }

                self.infer_expression(let_.expression(), &variables)
            }
            Expression::Number(number) => {
                Ok(types::Number::new(number.source_information().clone()).into())
            }
            Expression::Operation(operation) => {
                let type_: Type = types::Number::new(operation.source_information().clone()).into();

                let lhs = self.infer_expression(operation.lhs(), variables)?;
                self.equations.push(Equation::new(lhs, type_.clone()));
                let rhs = self.infer_expression(operation.rhs(), variables)?;
                self.equations.push(Equation::new(rhs, type_.clone()));

                Ok(type_)
            }
            Expression::Variable(variable) => variables
                .get(variable.name())
                .map(|type_| type_.clone())
                .ok_or(TypeInferenceError::VariableNotFound(
                    variable.source_information().clone(),
                )),
        }
    }

    fn reduce_equations(&mut self) -> Result<HashMap<usize, Type>, TypeInferenceError> {
        let mut substitutions = HashMap::<usize, Type>::new();

        while let Some(equation) = self.equations.pop() {
            let lhs = equation.lhs();
            let rhs = equation.rhs();

            match (lhs, rhs) {
                (Type::Variable(variable), _) => {
                    if let Type::Variable(another_variable) = rhs {
                        if variable.id() == another_variable.id() {
                            break;
                        }
                    }

                    for (_, substituted_type) in substitutions.iter_mut() {
                        *substituted_type =
                            substituted_type.clone().substitute_variable(variable, rhs);
                    }

                    for equation in self.equations.iter_mut() {
                        *equation = Equation::new(
                            equation.lhs().substitute_variable(variable, rhs),
                            equation.rhs().substitute_variable(variable, rhs),
                        )
                    }

                    substitutions.insert(variable.id(), rhs.clone());
                }
                (_, Type::Variable(_)) => {
                    self.equations.push(Equation::new(rhs.clone(), lhs.clone()))
                }
                (Type::Function(function1), Type::Function(function2)) => {
                    self.equations.push(Equation::new(
                        function1.argument().clone(),
                        function2.argument().clone(),
                    ));
                    self.equations.push(Equation::new(
                        function1.result().clone(),
                        function2.result().clone(),
                    ));
                }
                (Type::Number(_), Type::Number(_)) => {}
                (_, _) => {
                    return Err(TypeInferenceError::TypesNotMatched(
                        lhs.source_information().clone(),
                        rhs.source_information().clone(),
                    ))
                }
            }
        }

        Ok(substitutions)
    }
}
