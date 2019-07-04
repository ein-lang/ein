use crate::ast;
use std::collections::HashSet;

pub struct FreeVariableFinder {}

impl FreeVariableFinder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, function_definition: &ast::FunctionDefinition) -> Vec<String> {
        self.find_in_function_definition(function_definition, &HashSet::new())
    }

    fn find_in_function_definition(
        &self,
        function_definition: &ast::FunctionDefinition,
        variables: &HashSet<String>,
    ) -> Vec<String> {
        self.find_in_expression(
            function_definition.body(),
            &variables
                .into_iter()
                .map(|name| name.clone())
                .chain(
                    function_definition
                        .arguments()
                        .iter()
                        .map(|name| name.clone()),
                )
                .collect(),
        )
    }

    fn find_in_expression(
        &self,
        expression: &ast::Expression,
        variables: &HashSet<String>,
    ) -> Vec<String> {
        match expression {
            ast::Expression::Application(application) => self
                .find_in_expression(application.function(), variables)
                .into_iter()
                .chain(self.find_in_expression(application.argument(), variables))
                .collect(),
            ast::Expression::Let(let_) => {
                let mut variables = variables.clone();
                let mut free_variables = vec![];

                for definition in let_.definitions() {
                    if let ast::Definition::FunctionDefinition(function_definition) = definition {
                        variables.insert(function_definition.name().into());
                    }
                }

                for definition in let_.definitions() {
                    match definition {
                        ast::Definition::FunctionDefinition(function_definition) => {
                            free_variables.extend_from_slice(
                                &self.find_in_function_definition(function_definition, &variables),
                            );
                        }
                        ast::Definition::ValueDefinition(value_definition) => {
                            free_variables.extend_from_slice(
                                &self.find_in_expression(value_definition.body(), &variables),
                            );
                            variables.insert(value_definition.name().into());
                        }
                    }
                }

                free_variables
                    .extend_from_slice(&self.find_in_expression(let_.expression(), &variables));

                free_variables
            }
            ast::Expression::Number(_) => vec![],
            ast::Expression::Operation(operation) => self
                .find_in_expression(operation.lhs(), variables)
                .into_iter()
                .chain(self.find_in_expression(operation.rhs(), variables))
                .collect(),
            ast::Expression::Variable(variable) => {
                if variables.contains(variable.name()) {
                    vec![]
                } else {
                    vec![variable.name().into()]
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::FreeVariableFinder;
    use crate::ast::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn find_no_free_variables() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Variable::new("x", SourceInformation::dummy()),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            Vec::<String>::new()
        );
    }

    #[test]
    fn find_free_variables() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Variable::new("y", SourceInformation::dummy()),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            vec!["y".to_string()]
        );
    }

    #[test]
    fn find_no_free_variables_in_let_values() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Variable::new("x", SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("y", SourceInformation::dummy()),
                ),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            Vec::<String>::new()
        );
    }

    #[test]
    fn find_free_variables_in_let_values() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Variable::new("z", SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("y", SourceInformation::dummy())
                ),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            vec!["z".to_string()]
        );
    }

    #[test]
    fn find_no_free_variables_in_let_functions() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["y".into()],
                        Variable::new("y", SourceInformation::dummy()),
                        types::Function::new(
                            types::Variable::new(SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("f", SourceInformation::dummy())
                ),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            Vec::<String>::new()
        );
    }

    #[test]
    fn find_free_variables_in_let_functions() {
        assert_eq!(
            FreeVariableFinder::new().find(&FunctionDefinition::new(
                "f",
                vec!["x".into()],
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["y".into()],
                        Variable::new("z", SourceInformation::dummy()),
                        types::Function::new(
                            types::Variable::new(SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("f", SourceInformation::dummy())
                ),
                types::Function::new(
                    types::Variable::new(SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )),
            vec!["z".to_string()]
        );
    }
}
