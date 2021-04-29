use super::error::CompileError;
use super::list_type_configuration::ListTypeConfiguration;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_compiler::{TypeCompiler, NONE_TYPE_NAME};
use crate::ast::*;
use crate::types::Type;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TypeDefinitionCompiler {
    type_compiler: Arc<TypeCompiler>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    list_type_configuration: Arc<ListTypeConfiguration>,
}

impl TypeDefinitionCompiler {
    pub fn new(
        type_compiler: Arc<TypeCompiler>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        list_type_configuration: Arc<ListTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            type_compiler,
            reference_type_resolver,
            list_type_configuration,
        }
        .into()
    }

    pub fn compile(&self, module: &Module) -> Result<Vec<eir::ir::TypeDefinition>, CompileError> {
        let mut types = HashSet::new();

        module.transform_expressions(&mut |expression| -> Result<Expression, CompileError> {
            match expression {
                Expression::TypeCoercion(coercion) => {
                    if !self.is_variant(coercion.from())? && self.is_variant(coercion.to())? {
                        types.insert(coercion.from().clone());
                    }
                }
                Expression::Case(case) => {
                    for alternative in case.alternatives() {
                        types.insert(alternative.type_().clone());
                    }
                }
                _ => {}
            };

            Ok(expression.clone())
        })?;

        Ok(types
            .iter()
            .map(|type_| self.compile_type_definitions(type_))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    fn compile_type_definitions(
        &self,
        type_: &Type,
    ) -> Result<Vec<eir::ir::TypeDefinition>, CompileError> {
        Ok(match &self.reference_type_resolver.resolve(type_)? {
            Type::List(list) => vec![eir::ir::TypeDefinition::new(
                self.type_compiler.compile_list_type_name(list)?,
                eir::types::Record::new(vec![eir::types::Reference::new(
                    &self.list_type_configuration.list_type_name,
                )
                .into()]),
            )],
            Type::None(_) => vec![eir::ir::TypeDefinition::new(
                NONE_TYPE_NAME,
                eir::types::Record::new(vec![]),
            )],
            Type::Record(record) => vec![eir::ir::TypeDefinition::new(
                record.name(),
                self.type_compiler.compile_record(record)?,
            )],
            Type::Union(union) => union
                .types()
                .iter()
                .map(|type_| self.compile_type_definitions(type_))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
            Type::Any(_)
            | Type::Boolean(_)
            | Type::Function(_)
            | Type::Number(_)
            | Type::String(_) => vec![],
            Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }

    fn is_variant(&self, type_: &Type) -> Result<bool, CompileError> {
        Ok(self.reference_type_resolver.is_union(type_)?
            || self.reference_type_resolver.is_any(type_)?)
    }
}
