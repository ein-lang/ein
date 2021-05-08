use super::super::{
    error::CompileError, list_type_configuration::ListTypeConfiguration,
    name_generator::NameGenerator, reference_type_resolver::ReferenceTypeResolver,
};
use crate::{ast::*, types};
use std::sync::Arc;

pub struct ListCaseTransformer {
    first_rest_name_generator: NameGenerator,
    element_name_generator: NameGenerator,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    configuration: Arc<ListTypeConfiguration>,
}

impl ListCaseTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        configuration: Arc<ListTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            first_rest_name_generator: NameGenerator::new("$lc$firstRest"),
            element_name_generator: NameGenerator::new("$lc$firstRest"),
            reference_type_resolver,
            configuration,
        }
        .into()
    }

    pub fn transform(&self, case: &ListCase) -> Result<Expression, CompileError> {
        let source_information = case.source_information();
        let first_rest_type = types::Reference::new(
            &self.configuration.first_rest_type_name,
            source_information.clone(),
        );
        let none_type = types::None::new(source_information.clone());
        let element_type = self
            .reference_type_resolver
            .resolve_to_list(case.type_())?
            .unwrap()
            .element()
            .clone();

        let first_rest_name = self.first_rest_name_generator.generate();
        let element_name = self.element_name_generator.generate();

        Ok(Case::with_type(
            types::Union::new(
                vec![first_rest_type.clone().into(), none_type.clone().into()],
                source_information.clone(),
            ),
            &first_rest_name,
            Application::new(
                Variable::new(
                    &self.configuration.deconstruct_function_name,
                    source_information.clone(),
                ),
                case.argument().clone(),
                source_information.clone(),
            ),
            vec![
                Alternative::new(none_type, case.empty_alternative().clone()),
                Alternative::new(
                    first_rest_type,
                    Let::new(
                        vec![
                            VariableDefinition::new(
                                case.first_name(),
                                Case::with_type(
                                    types::Any::new(source_information.clone()),
                                    &element_name,
                                    Application::new(
                                        Variable::new(
                                            &self.configuration.first_function_name,
                                            source_information.clone(),
                                        ),
                                        Variable::new(&first_rest_name, source_information.clone()),
                                        source_information.clone(),
                                    ),
                                    vec![Alternative::new(
                                        element_type.clone(),
                                        Variable::new(&element_name, source_information.clone()),
                                    )],
                                    source_information.clone(),
                                ),
                                element_type,
                                source_information.clone(),
                            )
                            .into(),
                            VariableDefinition::new(
                                case.rest_name(),
                                Application::new(
                                    Variable::new(
                                        &self.configuration.rest_function_name,
                                        source_information.clone(),
                                    ),
                                    Variable::new(&first_rest_name, source_information.clone()),
                                    source_information.clone(),
                                ),
                                case.type_().clone(),
                                source_information.clone(),
                            )
                            .into(),
                        ],
                        case.non_empty_alternative().clone(),
                        source_information.clone(),
                    ),
                ),
            ],
            source_information.clone(),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::list_type_configuration::LIST_TYPE_CONFIGURATION, *};
    use crate::debug::*;

    fn create_list_case_transformer() -> Arc<ListCaseTransformer> {
        ListCaseTransformer::new(
            ReferenceTypeResolver::new(&Module::dummy()),
            LIST_TYPE_CONFIGURATION.clone(),
        )
    }

    #[test]
    fn transform() {
        insta::assert_debug_snapshot!(create_list_case_transformer().transform(&ListCase::new(
            Variable::new("xs", SourceInformation::dummy()),
            types::List::new(
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            ),
            "y",
            "ys",
            None::new(SourceInformation::dummy()),
            None::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        )));
    }
}
