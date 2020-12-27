use crate::ast::*;
use crate::types::Type;

pub struct ElementlessRecordTransformer {}

impl ElementlessRecordTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(&mut self, module: &Module) -> Module {
        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imports().to_vec(),
            module.foreign_declarations().to_vec(),
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
                                if record_type.elements().is_empty() {
                                    vec![VariableDefinition::new(
                                        type_definition.name(),
                                        RecordConstruction::new(
                                            record_type.clone(),
                                            Default::default(),
                                            record_type.source_information().clone(),
                                        ),
                                        record_type.clone(),
                                        record_type.source_information().clone(),
                                    )
                                    .into()]
                                } else {
                                    vec![]
                                }
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
    fn transform_elementless_record_type_definitions() {
        let record_type = types::Record::new("Foo", Default::default(), SourceInformation::dummy());

        assert_eq!(
            ElementlessRecordTransformer::new().transform(
                &Module::from_definitions_and_type_definitions(
                    vec![TypeDefinition::new("Foo", record_type.clone())],
                    vec![]
                )
            ),
            Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", record_type.clone())],
                vec![VariableDefinition::new(
                    "Foo",
                    RecordConstruction::new(
                        record_type.clone(),
                        Default::default(),
                        SourceInformation::dummy(),
                    ),
                    record_type,
                    SourceInformation::dummy(),
                )
                .into()]
            )
        );
    }
}
