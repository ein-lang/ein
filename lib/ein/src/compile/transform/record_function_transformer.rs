use crate::ast::*;
use crate::types::{self, Type};

pub struct RecordFunctionTransformer {}

impl RecordFunctionTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(&mut self, module: &Module) -> Module {
        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imports().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .cloned()
                .chain(
                    module
                        .type_definitions()
                        .iter()
                        .map(|type_definition| {
                            if let Type::Record(record_type) = type_definition.type_() {
                                record_type
                                    .elements()
                                    .iter()
                                    .map(|(key, element_type)| {
                                        let source_information = record_type.source_information();

                                        FunctionDefinition::new(
                                            format!("{}.{}", type_definition.name(), key),
                                            vec!["record".into()],
                                            RecordElementOperation::new(
                                                type_definition.type_().clone(),
                                                key,
                                                Variable::new("record", source_information.clone()),
                                                source_information.clone(),
                                            ),
                                            types::Function::new(
                                                type_definition.type_().clone(),
                                                element_type.clone(),
                                                source_information.clone(),
                                            ),
                                            source_information.clone(),
                                        )
                                        .into()
                                    })
                                    .collect()
                            } else {
                                vec![]
                            }
                        })
                        .flatten(),
                )
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::*;
    use crate::types;
    use pretty_assertions::assert_eq;

    #[test]
    fn transform_record_element_functions() {
        let record_type = types::Record::new(
            "Foo",
            vec![(
                "foo".into(),
                types::None::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
            SourceInformation::dummy(),
        );

        assert_eq!(
            RecordFunctionTransformer::new().transform(
                &Module::from_definitions_and_type_definitions(
                    vec![TypeDefinition::new("Foo", record_type.clone())],
                    vec![]
                )
            ),
            Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type.clone())],
                vec![FunctionDefinition::new(
                    "Foo.foo",
                    vec!["record".into()],
                    RecordElementOperation::new(
                        record_type.clone(),
                        "foo",
                        Variable::new("record", SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    types::Function::new(
                        record_type.clone(),
                        types::None::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into()]
            )
        );
    }
}
