use crate::ast::*;
use crate::types::{self, Type};

pub struct RecordIdQualifier {}

impl RecordIdQualifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn qualify(&self, module: &Module) -> Module {
        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imported_modules().to_vec(),
            module
                .type_definitions()
                .iter()
                .map(|type_definition| {
                    if let Type::Record(record) = type_definition.type_() {
                        TypeDefinition::new(
                            type_definition.name(),
                            types::Record::new(
                                module.path().fully_qualify_name(type_definition.name()),
                                record.elements().clone(),
                                record.source_information().clone(),
                            ),
                        )
                    } else {
                        type_definition.clone()
                    }
                })
                .collect(),
            module.definitions().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;
    use pretty_assertions::assert_eq;

    #[test]
    fn qualify() {
        assert_eq!(
            RecordIdQualifier::new().qualify(&Module::new(
                ModulePath::new(Package::new("Package", ""), vec!["A".into(), "B".into()]),
                Export::new(Default::default()),
                Default::default(),
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new("Foo", Default::default(), SourceInformation::dummy()),
                )],
                vec![],
            )),
            Module::new(
                ModulePath::new(Package::new("Package", ""), vec!["A".into(), "B".into()]),
                Export::new(Default::default()),
                Default::default(),
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new(
                        "Package().A.B.Foo",
                        Default::default(),
                        SourceInformation::dummy()
                    ),
                )],
                vec![],
            )
        );
    }
}
