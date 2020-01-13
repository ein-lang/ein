use super::super::reference_type_resolver::ReferenceTypeResolver;
use crate::ast::*;

pub fn resolve_function_reference_types(module: &Module) -> Module {
    let reference_type_resolver = ReferenceTypeResolver::new(module);

    module.convert_definitions(&mut |definition| match definition {
        Definition::FunctionDefinition(function_definition) => FunctionDefinition::new(
            function_definition.name(),
            function_definition.arguments().to_vec(),
            function_definition.body().clone(),
            reference_type_resolver.resolve(function_definition.type_()),
            function_definition.source_information().clone(),
        )
        .into(),
        Definition::ValueDefinition(value_definition) => ValueDefinition::new(
            value_definition.name(),
            value_definition.body().clone(),
            reference_type_resolver.resolve(value_definition.type_()),
            value_definition.source_information().clone(),
        )
        .into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::*;
    use crate::path::*;
    use crate::types;

    #[test]
    fn resolve_function_reference_types_() {
        assert_eq!(
            resolve_function_reference_types(&Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "Foo",
                    types::Number::new(SourceInformation::dummy()),
                )],
                vec![FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Reference::new("Foo", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                )
                .into()],
            )),
            Module::new(
                ModulePath::new(Package::new("", ""), vec![]),
                Export::new(Default::default()),
                vec![],
                vec![TypeDefinition::new(
                    "Foo",
                    types::Number::new(SourceInformation::dummy()),
                )],
                vec![FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                )
                .into()],
            )
        );
    }
}
