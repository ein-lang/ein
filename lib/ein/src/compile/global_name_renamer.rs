use super::error::CompileError;
use crate::ast::*;
use std::collections::HashMap;

pub struct GlobalNameRenamer {
    names: HashMap<String, String>,
}

impl GlobalNameRenamer {
    pub fn new(names: HashMap<String, String>) -> Self {
        Self { names }
    }

    pub fn rename(&self, module: &Module) -> Module {
        let module = module
            .convert_definitions(&mut |definition| -> Result<_, CompileError> {
                Ok(match definition {
                    Definition::FunctionDefinition(function_definition) => self
                        .rename_function_definition(function_definition, &self.names)
                        .into(),
                    Definition::ValueDefinition(value_definition) => self
                        .rename_value_definition(value_definition, &self.names)
                        .into(),
                })
            })
            .unwrap();

        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imported_modules().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .map(|definition| match definition {
                    Definition::FunctionDefinition(function_definition) => FunctionDefinition::new(
                        self.rename_name(function_definition.name(), &self.names),
                        function_definition.arguments().to_vec(),
                        function_definition.body().clone(),
                        function_definition.type_().clone(),
                        function_definition.source_information().clone(),
                    )
                    .into(),
                    Definition::ValueDefinition(value_definition) => ValueDefinition::new(
                        self.rename_name(value_definition.name(), &self.names),
                        value_definition.body().clone(),
                        value_definition.type_().clone(),
                        value_definition.source_information().clone(),
                    )
                    .into(),
                })
                .collect(),
        )
    }

    fn rename_function_definition(
        &self,
        function_definition: &FunctionDefinition,
        names: &HashMap<String, String>,
    ) -> FunctionDefinition {
        let mut names = names.clone();

        for name in function_definition.arguments() {
            names.remove(name);
        }

        FunctionDefinition::new(
            function_definition.name(),
            function_definition.arguments().to_vec(),
            function_definition
                .body()
                .convert_expressions(&mut |expression| -> Result<_, CompileError> {
                    Ok(self.rename_expression(expression, &names))
                })
                .unwrap(),
            function_definition.type_().clone(),
            function_definition.source_information().clone(),
        )
    }

    fn rename_value_definition(
        &self,
        value_definition: &ValueDefinition,
        names: &HashMap<String, String>,
    ) -> ValueDefinition {
        ValueDefinition::new(
            value_definition.name(),
            value_definition
                .body()
                .convert_expressions(&mut |expression| -> Result<_, CompileError> {
                    Ok(self.rename_expression(expression, &names))
                })
                .unwrap(),
            value_definition.type_().clone(),
            value_definition.source_information().clone(),
        )
    }

    fn rename_expression(
        &self,
        expression: &Expression,
        names: &HashMap<String, String>,
    ) -> Expression {
        match expression {
            Expression::Application(application) => Application::new(
                self.rename_expression(application.function(), names),
                self.rename_expression(application.argument(), names),
                application.source_information().clone(),
            )
            .into(),
            Expression::Case(case) => Case::with_type(
                case.type_().clone(),
                case.name(),
                self.rename_expression(case.argument(), names),
                {
                    let mut names = names.clone();

                    names.remove(case.name());

                    case.alternatives()
                        .iter()
                        .map(|alternative| {
                            Alternative::new(
                                alternative.type_().clone(),
                                self.rename_expression(alternative.expression(), &names),
                            )
                        })
                        .collect()
                },
                case.source_information().clone(),
            )
            .into(),
            Expression::If(if_) => If::new(
                self.rename_expression(if_.condition(), names),
                self.rename_expression(if_.then(), names),
                self.rename_expression(if_.else_(), names),
                if_.source_information().clone(),
            )
            .into(),
            Expression::Let(let_) => {
                let mut names = names.clone();

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            names.remove(function_definition.name());
                        }
                        Definition::ValueDefinition(value_definition) => {
                            if let_.has_functions() {
                                names.remove(value_definition.name());
                            }
                        }
                    }
                }

                let mut definitions = vec![];

                for definition in let_.definitions() {
                    definitions.push(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .rename_function_definition(function_definition, &names)
                            .into(),
                        Definition::ValueDefinition(value_definition) => {
                            let definition = self.rename_value_definition(value_definition, &names);

                            names.remove(value_definition.name());

                            definition.into()
                        }
                    })
                }

                Let::new(
                    definitions,
                    self.rename_expression(let_.expression(), &names),
                )
                .into()
            }
            Expression::Operation(operation) => Operation::with_type(
                operation.type_().clone(),
                operation.operator(),
                self.rename_expression(operation.lhs(), &names),
                self.rename_expression(operation.rhs(), &names),
                operation.source_information().clone(),
            )
            .into(),
            Expression::RecordConstruction(record_construction) => RecordConstruction::new(
                record_construction.type_().clone(),
                record_construction
                    .elements()
                    .iter()
                    .map(|(key, expression)| {
                        (key.clone(), self.rename_expression(expression, names))
                    })
                    .collect(),
                record_construction.source_information().clone(),
            )
            .into(),
            Expression::RecordElementOperation(operation) => RecordElementOperation::new(
                operation.type_().clone(),
                operation.key(),
                self.rename_expression(operation.argument(), names),
                operation.source_information().clone(),
            )
            .into(),
            Expression::Variable(variable) => Variable::new(
                self.rename_name(variable.name(), names),
                variable.source_information().clone(),
            )
            .into(),
            Expression::Boolean(_) | Expression::None(_) | Expression::Number(_) => {
                expression.clone()
            }
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }

    fn rename_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}
