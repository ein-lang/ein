use super::error::CompileError;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;

pub struct GlobalNameRenamer<'a> {
    names: &'a HashMap<String, String>,
}

impl<'a> GlobalNameRenamer<'a> {
    pub fn new(names: &'a HashMap<String, String>) -> Self {
        Self { names }
    }

    pub fn rename(&self, module: &Module) -> Module {
        let module = module
            .convert_definitions(&mut |definition| -> Result<_, ()> {
                Ok(match definition {
                    Definition::FunctionDefinition(function_definition) => self
                        .rename_function_definition(function_definition, &self.names)
                        .into(),
                    Definition::ValueDefinition(value_definition) => self
                        .rename_value_definition(value_definition, &self.names)
                        .into(),
                })
            })
            .unwrap()
            .convert_types(&mut |type_| -> Result<_, ()> {
                Ok(match type_ {
                    Type::Record(record) => types::Record::new(
                        self.rename_name(record.name(), &self.names),
                        record.elements().clone(),
                        record.source_information().clone(),
                    )
                    .into(),
                    Type::Reference(reference) => types::Reference::new(
                        self.rename_name(reference.name(), &self.names),
                        reference.source_information().clone(),
                    )
                    .into(),
                    _ => type_.clone(),
                })
            })
            .unwrap();

        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imports().to_vec(),
            module
                .type_definitions()
                .iter()
                .map(|type_definition| {
                    TypeDefinition::new(
                        self.rename_name(type_definition.name(), &self.names),
                        type_definition.type_().clone(),
                    )
                })
                .collect(),
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
                .convert_expressions(&mut |expression| -> Result<_, ()> {
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
            Expression::List(list) => List::new(
                list.elements()
                    .iter()
                    .map(|element| match element {
                        ListElement::Multiple(expression) => {
                            ListElement::Multiple(self.rename_expression(expression, &names))
                        }
                        ListElement::Single(expression) => {
                            ListElement::Single(self.rename_expression(expression, &names))
                        }
                    })
                    .collect(),
                list.source_information().clone(),
            )
            .into(),
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
                operation.variable(),
                {
                    let names = names.clone();
                    names.remove(operation.variable());
                    self.rename_expression(operation.expression(), &names)
                },
                operation.source_information().clone(),
            )
            .into(),
            Expression::RecordUpdate(record_update) => RecordUpdate::new(
                record_update.type_().clone(),
                self.rename_expression(record_update.argument(), names),
                record_update
                    .elements()
                    .iter()
                    .map(|(key, expression)| {
                        (key.clone(), self.rename_expression(expression, names))
                    })
                    .collect(),
                record_update.source_information().clone(),
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
            Expression::TypeCoercion(_) => unreachable!(),
        }
    }

    fn rename_name(&self, name: &str, names: &HashMap<String, String>) -> String {
        names.get(name).cloned().unwrap_or_else(|| name.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::package::Package;
    use crate::path::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn rename_nothing() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameRenamer::new(&Default::default()).rename(&module),
            module
        );
    }

    #[test]
    fn rename_names_in_value_definitions() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![],
            vec![ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameRenamer::new(&vec![("x".into(), "y".into())].into_iter().collect())
                .rename(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![ValueDefinition::new(
                    "y",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()]
            )
        );
    }

    #[test]
    fn do_not_rename_names_in_export_statements() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(vec!["x".into()].into_iter().collect()),
            vec![],
            vec![],
            vec![],
        );

        assert_eq!(
            GlobalNameRenamer::new(&vec![("x".into(), "y".into())].into_iter().collect())
                .rename(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(vec!["x".into()].into_iter().collect()),
                vec![],
                vec![],
                vec![],
            )
        );
    }

    #[test]
    fn rename_names_in_type_definitions() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![TypeDefinition::new(
                "x",
                types::None::new(SourceInformation::dummy()),
            )],
            vec![],
        );

        assert_eq!(
            GlobalNameRenamer::new(&vec![("x".into(), "y".into())].into_iter().collect())
                .rename(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "y",
                    types::None::new(SourceInformation::dummy()),
                )],
                vec![],
            )
        );
    }

    #[test]
    fn rename_reference_types() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![TypeDefinition::new(
                "x",
                types::Reference::new("z", SourceInformation::dummy()),
            )],
            vec![ValueDefinition::new(
                "y",
                None::new(SourceInformation::dummy()),
                types::Reference::new("z", SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        );

        assert_eq!(
            GlobalNameRenamer::new(&vec![("z".into(), "v".into())].into_iter().collect())
                .rename(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "x",
                    types::Reference::new("v", SourceInformation::dummy()),
                )],
                vec![ValueDefinition::new(
                    "y",
                    None::new(SourceInformation::dummy()),
                    types::Reference::new("v", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()],
            )
        );
    }

    #[test]
    fn rename_record_types() {
        let module = Module::new(
            ModulePath::new(Package::new("M", ""), vec![]),
            Export::new(Default::default()),
            vec![],
            vec![TypeDefinition::new(
                "x",
                types::Record::new("y", Default::default(), SourceInformation::dummy()),
            )],
            vec![],
        );

        assert_eq!(
            GlobalNameRenamer::new(&vec![("y".into(), "z".into())].into_iter().collect())
                .rename(&module),
            Module::new(
                ModulePath::new(Package::new("M", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "x",
                    types::Record::new("z", Default::default(), SourceInformation::dummy()),
                )],
                vec![],
            )
        );
    }
}
