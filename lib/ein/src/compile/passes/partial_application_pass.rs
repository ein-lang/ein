use super::super::error::CompileError;
use super::super::name_generator::NameGenerator;
use super::super::pass::Pass;
use crate::ast::*;
use crate::debug::*;
use crate::types::Type;

pub struct PartialApplicationPass {
    name_generator: NameGenerator,
}

impl PartialApplicationPass {
    pub fn new() -> Self {
        Self {
            name_generator: NameGenerator::new("pa_argument_"),
        }
    }

    pub fn compile(&mut self, module: &Module) -> Result<Module, CompileError> {
        module
            .convert_definitions(&mut |definition| -> Result<_, CompileError> {
                if let Definition::ValueDefinition(value_definition) = definition {
                    Ok(self.compile_value_definition(value_definition))
                } else {
                    Ok(definition.clone())
                }
            })?
            .convert_definitions(&mut |definition| -> Result<_, CompileError> {
                if let Definition::FunctionDefinition(function_definition) = definition {
                    Ok(self.compile_function_definition(function_definition).into())
                } else {
                    Ok(definition.clone())
                }
            })
    }

    fn compile_value_definition(&mut self, value_definition: &ValueDefinition) -> Definition {
        if let Type::Function(_) = value_definition.type_() {
            FunctionDefinition::new(
                value_definition.name(),
                vec![],
                value_definition.body().clone(),
                value_definition.type_().clone(),
                value_definition.source_information().clone(),
            )
            .into()
        } else {
            value_definition.clone().into()
        }
    }

    fn compile_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> FunctionDefinition {
        let function_type = function_definition.type_().to_function().unwrap();

        if function_definition.arguments().len() == function_type.arguments().len() {
            function_definition.clone()
        } else {
            let omitted_arguments = (0..(function_type.arguments().len()
                - function_definition.arguments().len()))
                .map(|_| self.name_generator.generate())
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
            Expression::Case(case) => Case::with_type(
                case.type_().clone(),
                case.name(),
                case.argument().clone(),
                case.alternatives()
                    .iter()
                    .map(|alternative| {
                        Alternative::new(
                            alternative.type_().clone(),
                            self.apply_arguments_recursively(alternative.expression(), arguments),
                        )
                    })
                    .collect(),
                case.source_information().clone(),
            )
            .into(),
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
            Expression::RecordElementOperation(operation) => {
                self.apply_arguments(expression, arguments, operation.source_information())
            }
            Expression::Variable(variable) => {
                self.apply_arguments(expression, arguments, variable.source_information())
            }
            Expression::Boolean(_)
            | Expression::List(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Operation(_)
            | Expression::RecordConstruction(_)
            | Expression::RecordUpdate(_)
            | Expression::TypeCoercion(_) => unreachable!(),
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

impl Pass for PartialApplicationPass {
    fn compile(&mut self, module: &Module) -> Result<Module, CompileError> {
        self.compile(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn complement_an_omitted_argument_of_value_definition() {
        assert_eq!(
            PartialApplicationPass::new().compile(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "f",
                    Variable::new("g", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Ok(Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["pa_argument_0".into()],
                Application::new(
                    Variable::new("g", SourceInformation::dummy()),
                    Variable::new("pa_argument_0", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()]))
        );
    }

    #[test]
    fn complement_2_omitted_arguments_of_value_definition() {
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
            PartialApplicationPass::new().compile(&Module::from_definitions(vec![
                ValueDefinition::new(
                    "f",
                    Variable::new("g", SourceInformation::dummy()),
                    function_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()
            ])),
            Ok(Module::from_definitions(vec![FunctionDefinition::new(
                "f",
                vec!["pa_argument_0".into(), "pa_argument_1".into()],
                Application::new(
                    Application::new(
                        Variable::new("g", SourceInformation::dummy()),
                        Variable::new("pa_argument_0", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    Variable::new("pa_argument_1", SourceInformation::dummy()),
                    SourceInformation::dummy(),
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
            PartialApplicationPass::new().compile(&Module::from_definitions(vec![
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
            PartialApplicationPass::new().compile(&Module::from_definitions(vec![
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
