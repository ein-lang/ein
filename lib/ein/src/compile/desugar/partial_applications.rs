use super::super::error::CompileError;
use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::debug::*;
use crate::types::{self, Type};

pub struct PartialApplicationDesugarer {
    function_name_generator: NameGenerator,
    argument_name_generator: NameGenerator,
}

impl PartialApplicationDesugarer {
    pub fn new() -> Self {
        Self {
            function_name_generator: NameGenerator::new("pa_function_"),
            argument_name_generator: NameGenerator::new("pa_argument_"),
        }
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        module
            .convert_definitions(&mut |definition| -> Result<_, CompileError> {
                if let Definition::ValueDefinition(value_definition) = definition {
                    Ok(self.desugar_value_definition(value_definition).into())
                } else {
                    Ok(definition.clone())
                }
            })?
            .convert_definitions(&mut |definition| -> Result<_, CompileError> {
                if let Definition::FunctionDefinition(function_definition) = definition {
                    Ok(self.desugar_function_definition(function_definition).into())
                } else {
                    Ok(definition.clone())
                }
            })
    }

    fn desugar_value_definition(&mut self, value_definition: &ValueDefinition) -> ValueDefinition {
        if let Type::Function(function_type) = value_definition.type_() {
            ValueDefinition::new(
                value_definition.name(),
                self.convert_partial_applications(value_definition.body(), function_type),
                value_definition.type_().clone(),
                value_definition.source_information().clone(),
            )
        } else {
            value_definition.clone()
        }
    }

    fn convert_partial_applications(
        &mut self,
        expression: &Expression,
        function_type: &types::Function,
    ) -> Expression {
        match expression {
            Expression::Application(application) => {
                let name = self.function_name_generator.generate();

                Let::new(
                    vec![FunctionDefinition::new(
                        &name,
                        vec![],
                        application.clone(),
                        function_type.clone(),
                        application.source_information().clone(),
                    )
                    .into()],
                    Variable::new(name, application.source_information().clone()),
                )
                .into()
            }
            Expression::If(if_) => If::new(
                if_.condition().clone(),
                self.convert_partial_applications(if_.then(), function_type),
                self.convert_partial_applications(if_.else_(), function_type),
                if_.source_information().clone(),
            )
            .into(),
            Expression::Let(let_) => Let::new(
                let_.definitions().to_vec(),
                self.convert_partial_applications(let_.expression(), function_type),
            )
            .into(),
            Expression::Variable(_) => expression.clone(),
            Expression::Boolean(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Operation(_)
            | Expression::RecordConstruction(_)
            | Expression::RecordUpdate(_) => unreachable!(),
        }
    }

    fn desugar_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> FunctionDefinition {
        let function_type = function_definition.type_().to_function().unwrap();

        if function_definition.arguments().len() == function_type.arguments().len() {
            function_definition.clone()
        } else {
            let omitted_arguments = (0..(function_type.arguments().len()
                - function_definition.arguments().len()))
                .map(|_| self.argument_name_generator.generate())
                .collect::<Vec<_>>();

            FunctionDefinition::new(
                function_definition.name(),
                function_definition
                    .arguments()
                    .iter()
                    .chain(&omitted_arguments)
                    .cloned()
                    .collect(),
                self.apply_arguments_recursively(
                    function_definition.body(),
                    &omitted_arguments
                        .into_iter()
                        .map(|argument| {
                            Variable::new(
                                argument,
                                function_definition.source_information().clone(),
                            )
                        })
                        .collect::<Vec<Variable>>(),
                ),
                function_definition.type_().clone(),
                function_definition.source_information().clone(),
            )
        }
    }

    fn apply_arguments_recursively(
        &self,
        expression: &Expression,
        arguments: &[Variable],
    ) -> Expression {
        match expression {
            Expression::Application(application) => {
                self.apply_arguments(expression, arguments, application.source_information())
            }
            Expression::If(if_) => If::new(
                if_.condition().clone(),
                self.apply_arguments_recursively(if_.then(), arguments),
                self.apply_arguments_recursively(if_.else_(), arguments),
                if_.source_information().clone(),
            )
            .into(),
            Expression::Let(let_) => Let::new(
                let_.definitions().to_vec(),
                self.apply_arguments_recursively(let_.expression(), arguments),
            )
            .into(),
            Expression::Variable(variable) => {
                self.apply_arguments(expression, arguments, variable.source_information())
            }
            Expression::Boolean(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Operation(_)
            | Expression::RecordConstruction(_)
            | Expression::RecordUpdate(_) => unreachable!(),
        }
    }

    fn apply_arguments(
        &self,
        expression: &Expression,
        arguments: &[Variable],
        source_information: &SourceInformation,
    ) -> Expression {
        arguments
            .iter()
            .fold(expression.clone(), |application, argument| {
                Application::new(application, argument.clone(), source_information.clone()).into()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn do_not_convert_value_definitions_of_variables() {
        let value_definition = ValueDefinition::new(
            "f",
            Variable::new("g", SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        );

        assert_eq!(
            PartialApplicationDesugarer::new().desugar(&Module::from_definitions(vec![
                value_definition.clone().into()
            ])),
            Ok(Module::from_definitions(vec![value_definition.into()]))
        );
    }

    #[test]
    fn convert_value_definitions_of_applications() {
        let function_type = types::Function::new(
            types::Number::new(SourceInformation::dummy()),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        );

        assert_eq!(
            PartialApplicationDesugarer::new().desugar(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "f",
                    Application::new(
                        Variable::new("g", SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    function_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Ok(Module::from_definitions(vec![ValueDefinition::new(
                "f",
                Let::new(
                    vec![FunctionDefinition::new(
                        "pa_function_0",
                        vec!["pa_argument_0".into()],
                        Application::new(
                            Application::new(
                                Variable::new("g", SourceInformation::dummy()),
                                Variable::new("x", SourceInformation::dummy()),
                                SourceInformation::dummy(),
                            ),
                            Variable::new("pa_argument_0", SourceInformation::dummy()),
                            SourceInformation::dummy(),
                        ),
                        function_type.clone(),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("pa_function_0", SourceInformation::dummy())
                ),
                function_type.clone(),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn complement_an_omitted_argument_of_function_definition() {
        let function_type = types::Function::new(
            types::Number::new(SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        );

        assert_eq!(
            PartialApplicationDesugarer::new().desugar(&Module::from_definitions(vec![
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("g", SourceInformation::dummy()),
                    function_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Ok(Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["x".into(), "pa_argument_0".into()],
                Application::new(
                    Variable::new("g", SourceInformation::dummy()),
                    Variable::new("pa_argument_0", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                function_type,
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn complement_2_omitted_arguments_of_function_definition() {
        let function_type = types::Function::new(
            types::Number::new(SourceInformation::dummy()),
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            ),
            SourceInformation::dummy(),
        );

        assert_eq!(
            PartialApplicationDesugarer::new().desugar(&Module::from_definitions(vec![
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("g", SourceInformation::dummy()),
                    function_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Ok(Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["x".into(), "pa_argument_0".into(), "pa_argument_1".into()],
                Application::new(
                    Application::new(
                        Variable::new("g", SourceInformation::dummy()),
                        Variable::new("pa_argument_0", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Variable::new("pa_argument_1", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                function_type,
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }
}
