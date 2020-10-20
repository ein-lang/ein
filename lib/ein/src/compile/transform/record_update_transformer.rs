use super::super::error::CompileError;
use super::super::name_generator::NameGenerator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use crate::ast::*;

pub struct RecordUpdateTransformer {
    name_generator: NameGenerator,
}

impl RecordUpdateTransformer {
    pub fn new() -> Self {
        Self {
            name_generator: NameGenerator::new("record_update_argument_"),
        }
    }

    pub fn transform(&mut self, module: &Module) -> Result<Module, CompileError> {
        let reference_type_resolver = ReferenceTypeResolver::new(module);

        module.convert_expressions(&mut |expression| -> Result<Expression, CompileError> {
            if let Expression::RecordUpdate(record_update) = expression {
                let type_ = reference_type_resolver.resolve(record_update.type_())?;
                let record_type = type_.to_record().unwrap();
                let source_information = record_update.source_information();
                let name = self.name_generator.generate();

                Ok(Let::new(
                    vec![ValueDefinition::new(
                        &name,
                        record_update.argument().clone(),
                        record_update.type_().clone(),
                        source_information.clone(),
                    )
                    .into()],
                    RecordConstruction::new(
                        record_update.type_().clone(),
                        record_type
                            .elements()
                            .iter()
                            .map(|(key, _)| {
                                (
                                    key.clone(),
                                    RecordElementOperation::new(
                                        record_update.type_().clone(),
                                        key,
                                        Variable::new(&name, source_information.clone()),
                                        "$element",
                                        Variable::new("$element", source_information.clone()),
                                        source_information.clone(),
                                    )
                                    .into(),
                                )
                            })
                            .chain(record_update.elements().clone())
                            .collect(),
                        source_information.clone(),
                    ),
                )
                .into())
            } else {
                Ok(expression.clone())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn transform_record_update() {
        let record_type = types::Record::new(
            "Foo",
            vec![
                (
                    "foo".into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ),
                (
                    "bar".into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ),
            ]
            .into_iter()
            .collect(),
            SourceInformation::dummy(),
        );
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        assert_eq!(
            RecordUpdateTransformer::new().transform(
                &Module::from_definitions_and_type_definitions(
                    vec![TypeDefinition::new("Foo", record_type.clone())],
                    vec![ValueDefinition::new(
                        "x",
                        RecordUpdate::new(
                            reference_type.clone(),
                            Variable::new("foo", SourceInformation::dummy()),
                            vec![("bar".into(), None::new(SourceInformation::dummy()).into())]
                                .into_iter()
                                .collect(),
                            SourceInformation::dummy()
                        ),
                        reference_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into()]
                )
            ),
            Ok(Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type.clone())],
                vec![ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
                            "record_update_argument_0",
                            Variable::new("foo", SourceInformation::dummy()),
                            reference_type.clone(),
                            SourceInformation::dummy(),
                        )
                        .into()],
                        RecordConstruction::new(
                            reference_type.clone(),
                            vec![
                                (
                                    "foo".into(),
                                    RecordElementOperation::new(
                                        reference_type.clone(),
                                        "foo",
                                        Variable::new(
                                            "record_update_argument_0",
                                            SourceInformation::dummy()
                                        ),
                                        "$element",
                                        Variable::new("$element", SourceInformation::dummy()),
                                        SourceInformation::dummy(),
                                    )
                                    .into()
                                ),
                                ("bar".into(), None::new(SourceInformation::dummy()).into())
                            ]
                            .into_iter()
                            .collect(),
                            SourceInformation::dummy()
                        ),
                    ),
                    reference_type.clone(),
                    SourceInformation::dummy(),
                )
                .into()]
            ))
        );
    }
}
