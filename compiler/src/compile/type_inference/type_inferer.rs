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
        self.reduce_equations()?;
        Ok(module.clone())
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
                    variables.insert(
                        value_definition.name(),
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
                    self.infer_value_definition(value_definition, &variables)
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
        let mut variables = variables.clone();
        let mut function_type = function_definition.type_();

        for (index, argument) in function_definition.arguments().iter().enumerate() {
            variables.insert(argument, function_type.argument().clone());

            if index == function_definition.arguments().len() - 1 {
                continue;
            }

            if let Type::Function(function) = function_type.result() {
                function_type = function;
            } else {
                return Err(TypeInferenceError::new("type inference error".into()));
            }
        }

        let type_ = self.infer_expression(function_definition.body(), &variables);
        self.equations
            .push(Equation::new(type_, function_type.result().clone()));

        Ok(())
    }

    fn infer_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<&str, Type>,
    ) {
        let type_ = self.infer_expression(value_definition.body(), &variables);
        self.equations
            .push(Equation::new(type_, value_definition.type_().clone()));
    }

    fn infer_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<&str, Type>,
    ) -> Type {
        match expression {
            Expression::Application(application) => {
                let function = self.infer_expression(application.function(), variables);
                let argument = self.infer_expression(application.argument(), variables);
                let result = Type::Variable(types::Variable::new());

                self.equations.push(Equation::new(
                    function,
                    Type::Function(types::Function::new(argument, result.clone())),
                ));

                result
            }
            Expression::Number(_) => Type::Number,
            Expression::Operation(operation) => {
                let lhs = self.infer_expression(operation.lhs(), variables);
                self.equations.push(Equation::new(lhs, Type::Number));
                let rhs = self.infer_expression(operation.rhs(), variables);
                self.equations.push(Equation::new(rhs, Type::Number));
                Type::Number
            }
            Expression::Variable(variable) => variables[variable.as_str()].clone(),
        }
    }

    fn reduce_equations(&mut self) -> Result<(), TypeInferenceError> {
        let mut substitutions = HashMap::<usize, Type>::new();

        while let Some(equation) = self.equations.pop() {
            match (equation.lhs(), equation.rhs()) {
                (Type::Variable(variable), type_) => {
                    if let Type::Variable(another_variable) = type_ {
                        if variable.id() == another_variable.id() {
                            break;
                        }
                    }

                    for (_, substituted_type) in substitutions.iter_mut() {
                        *substituted_type = substituted_type
                            .clone()
                            .substitute_variable(variable, type_);
                    }

                    for equation in self.equations.iter_mut() {
                        *equation = Equation::new(
                            equation.lhs().substitute_variable(variable, type_),
                            equation.rhs().substitute_variable(variable, type_),
                        )
                    }

                    substitutions.insert(variable.id(), type_.clone());
                }
                (type_, Type::Variable(variable)) => self
                    .equations
                    .push(Equation::new(variable.clone().into(), type_.clone())),
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
                (Type::Number, Type::Number) => {}
                (_, _) => return Err(TypeInferenceError::new("type inference error".into())),
            }
        }

        Ok(())
    }
}
