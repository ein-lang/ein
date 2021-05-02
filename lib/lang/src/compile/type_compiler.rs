use super::error::CompileError;
use super::list_type_configuration::ListTypeConfiguration;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_id_calculator::TypeIdCalculator;
use crate::types::{self, Type};
use std::sync::Arc;

pub const NONE_TYPE_NAME: &str = "ein_None";
pub const THUNK_ARGUMENT_TYPE_NAME: &str = "ein_thunk_argument";

pub struct TypeCompiler {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_id_calculator: Arc<TypeIdCalculator>,
    list_type_configuration: Arc<ListTypeConfiguration>,
}

impl TypeCompiler {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_id_calculator: Arc<TypeIdCalculator>,
        list_type_configuration: Arc<ListTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            type_id_calculator,
            list_type_configuration,
        }
        .into()
    }

    pub fn compile(&self, type_: &Type) -> Result<eir::types::Type, CompileError> {
        Ok(match type_ {
            Type::Any(_) => self.compile_any(),
            Type::Boolean(_) => self.compile_boolean().into(),
            Type::Function(function) => eir::types::Function::new(
                self.compile(function.argument())?,
                self.compile(function.result())?,
            )
            .into(),
            Type::List(_) => self.compile_any_list().into(),
            Type::None(_) => self.compile_none().into(),
            Type::Number(_) => eir::types::Primitive::Number.into(),
            Type::Record(record) => eir::types::Reference::new(record.name()).into(),
            Type::Reference(reference) => self.compile_reference(reference)?,
            Type::String(_) => self.compile_string(),
            Type::Union(_) => self.compile_union(),
            Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }

    fn compile_reference(
        &self,
        reference: &types::Reference,
    ) -> Result<eir::types::Type, CompileError> {
        self.compile(&self.reference_type_resolver.resolve_reference(reference)?)
    }

    pub fn compile_function(
        &self,
        type_: &types::Type,
    ) -> Result<eir::types::Function, CompileError> {
        Ok(self.compile(type_)?.into_function().unwrap())
    }

    pub fn compile_list(&self, list: &types::List) -> Result<eir::types::Reference, CompileError> {
        Ok(eir::types::Reference::new(format!(
            "ein_List_{:x}",
            self.type_id_calculator.calculate(list.element())?
        )))
    }

    pub fn compile_any_list(&self) -> eir::types::Reference {
        eir::types::Reference::new(&self.list_type_configuration.list_type_name)
    }

    pub fn compile_record(
        &self,
        record: &types::Record,
    ) -> Result<eir::types::Record, CompileError> {
        Ok(eir::types::Record::new(
            record
                .elements()
                .iter()
                .map(|(_, type_)| self.compile(type_))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    pub fn compile_union(&self) -> eir::types::Type {
        eir::types::Type::Variant
    }

    pub fn compile_any(&self) -> eir::types::Type {
        eir::types::Type::Variant
    }

    pub fn compile_boolean(&self) -> eir::types::Primitive {
        eir::types::Primitive::Boolean
    }

    pub fn compile_string(&self) -> eir::types::Type {
        eir::types::Type::String
    }

    pub fn compile_none(&self) -> eir::types::Reference {
        eir::types::Reference::new(NONE_TYPE_NAME)
    }

    pub fn compile_thunk_argument(&self) -> eir::types::Reference {
        eir::types::Reference::new(THUNK_ARGUMENT_TYPE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::super::list_type_configuration::LIST_TYPE_CONFIGURATION;
    use super::*;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use pretty_assertions::assert_eq;

    fn create_type_compiler() -> Arc<TypeCompiler> {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_id_calculator = TypeIdCalculator::new(reference_type_resolver.clone());

        TypeCompiler::new(
            reference_type_resolver,
            type_id_calculator,
            LIST_TYPE_CONFIGURATION.clone(),
        )
    }

    #[test]
    fn compile_number_type() {
        assert_eq!(
            create_type_compiler().compile(&types::Number::new(SourceInformation::dummy()).into()),
            Ok(eir::types::Primitive::Number.into())
        );
    }

    #[test]
    fn compile_function_type() {
        assert_eq!(
            create_type_compiler().compile(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into()
            ),
            Ok(eir::types::Function::new(
                eir::types::Primitive::Number,
                eir::types::Primitive::Number
            )
            .into())
        );
    }

    #[test]
    fn compile_record_type() {
        let reference_type = types::Reference::new("Foo", SourceInformation::dummy());
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "Foo",
                    types::Record::new("Bar", Default::default(), SourceInformation::dummy()),
                )],
                vec![],
            ));
        let type_id_calculator = TypeIdCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(
                reference_type_resolver,
                type_id_calculator,
                LIST_TYPE_CONFIGURATION.clone()
            )
            .compile(&reference_type.into()),
            Ok(eir::types::Reference::new("Bar").into())
        );
    }

    #[test]
    fn compile_union_type() {
        assert_eq!(
            create_type_compiler().compile(
                &types::Union::new(
                    vec![
                        types::Number::new(SourceInformation::dummy()).into(),
                        types::Boolean::new(SourceInformation::dummy()).into(),
                    ],
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(eir::types::Type::Variant)
        );
    }

    #[test]
    fn compile_any_type() {
        assert_eq!(
            create_type_compiler().compile(&types::Any::new(SourceInformation::dummy()).into(),),
            Ok(eir::types::Type::Variant)
        );
    }

    #[test]
    fn compile_list_type() {
        let reference_type_resolver =
            ReferenceTypeResolver::new(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new(
                    "GenericList",
                    types::Record::new(
                        "GenericList",
                        Default::default(),
                        SourceInformation::dummy(),
                    ),
                )],
                vec![],
            ));
        let type_id_calculator = TypeIdCalculator::new(reference_type_resolver.clone());

        assert_eq!(
            TypeCompiler::new(
                reference_type_resolver,
                type_id_calculator,
                LIST_TYPE_CONFIGURATION.clone()
            )
            .compile(
                &types::List::new(
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(eir::types::Reference::new(&LIST_TYPE_CONFIGURATION.list_type_name).into())
        );
    }
}
