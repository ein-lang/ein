mod desugar;
mod error;
mod expression_compiler;
mod expression_type_extractor;
mod module_compiler;
mod module_environment_creator;
mod module_interface_compiler;
mod name_generator;
mod name_qualifier;
mod reference_type_resolver;
mod type_compiler;
mod type_equality_checker;
mod type_inference;
mod union_tag_calculator;

use crate::ast;
use crate::path::ModulePath;
use desugar::{desugar_with_types, desugar_without_types};
use error::CompileError;
use expression_compiler::ExpressionCompiler;
use module_compiler::ModuleCompiler;
use module_interface_compiler::ModuleInterfaceCompiler;
use name_qualifier::NameQualifier;
use reference_type_resolver::ReferenceTypeResolver;
use type_compiler::TypeCompiler;
use type_inference::infer_types;
use union_tag_calculator::UnionTagCalculator;

const SOURCE_MAIN_FUNCTION_NAME: &str = "main";
const OBJECT_MAIN_FUNCTION_NAME: &str = "ein_main";
const OBJECT_INIT_FUNCTION_NAME: &str = "ein_init";

pub fn compile(module: &ast::Module) -> Result<(Vec<u8>, ast::ModuleInterface), CompileError> {
    let module = desugar_with_types(&infer_types(&desugar_without_types(module)?)?)?;
    let name_qualifier = NameQualifier::new(
        &module,
        vec![(
            SOURCE_MAIN_FUNCTION_NAME.into(),
            OBJECT_MAIN_FUNCTION_NAME.into(),
        )]
        .into_iter()
        .collect(),
    );

    let reference_type_resolver = ReferenceTypeResolver::new(&module);
    let union_tag_calculator = UnionTagCalculator::new(&reference_type_resolver);
    let type_compiler = TypeCompiler::new(&reference_type_resolver, &union_tag_calculator);
    let expression_compiler = ExpressionCompiler::new(
        &reference_type_resolver,
        &union_tag_calculator,
        &type_compiler,
    );

    Ok((
        ssf_llvm::compile(
            &name_qualifier.qualify_core_module(
                &ModuleCompiler::new(&module, &expression_compiler, &type_compiler).compile()?,
            ),
            &ssf_llvm::CompileConfiguration::new(
                if module
                    .definitions()
                    .iter()
                    .any(|definition| definition.name() == SOURCE_MAIN_FUNCTION_NAME)
                {
                    OBJECT_INIT_FUNCTION_NAME.into()
                } else {
                    convert_path_to_initializer_name(module.path())
                },
                module
                    .imported_modules()
                    .iter()
                    .map(|module_interface| {
                        convert_path_to_initializer_name(module_interface.path())
                    })
                    .collect(),
                None,
                None,
            ),
        )?,
        ModuleInterfaceCompiler::new().compile(&module)?,
    ))
}

fn convert_path_to_initializer_name(module_path: &ModulePath) -> String {
    module_path.fully_qualify_name("$init")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::*;
    use crate::types;

    #[test]
    fn compile_constant_initialized_with_operation() {
        assert!(compile(&Module::from_definitions(vec![
            ValueDefinition::new(
                "x",
                Number::new(42.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into(),
            ValueDefinition::new(
                "y",
                Operation::new(
                    Operator::Add,
                    Variable::new("x", SourceInformation::dummy()),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()
        ]))
        .is_ok());
    }

    #[test]
    fn compile_record_construction() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        assert!(compile(&Module::from_definitions_and_type_definitions(
            vec![TypeDefinition::new(
                "Foo",
                types::Record::new(
                    "Foo",
                    vec![(
                        "foo".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    )]
                    .into_iter()
                    .collect(),
                    SourceInformation::dummy(),
                ),
            )],
            vec![ValueDefinition::new(
                "x",
                RecordConstruction::new(
                    reference_type.clone(),
                    vec![(
                        "foo".into(),
                        Number::new(42.0, SourceInformation::dummy()).into()
                    )]
                    .into_iter()
                    .collect(),
                    SourceInformation::dummy(),
                ),
                reference_type.clone(),
                SourceInformation::dummy(),
            )
            .into()],
        ))
        .is_ok());
    }

    #[test]
    fn compile_record_element_access() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());

        assert!(compile(&Module::from_definitions_and_type_definitions(
            vec![TypeDefinition::new(
                "Foo",
                types::Record::new(
                    "Foo",
                    vec![(
                        "foo".into(),
                        types::Number::new(SourceInformation::dummy()).into()
                    )]
                    .into_iter()
                    .collect(),
                    SourceInformation::dummy(),
                ),
            )],
            vec![ValueDefinition::new(
                "x",
                Application::new(
                    Variable::new("Foo.foo", SourceInformation::dummy()),
                    RecordConstruction::new(
                        reference_type.clone(),
                        vec![(
                            "foo".into(),
                            Number::new(42.0, SourceInformation::dummy()).into()
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy(),
            )
            .into()],
        ))
        .is_ok());
    }
}
