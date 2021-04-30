use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_compiler::{TypeCompiler, NONE_TYPE_NAME};
use crate::ast::*;
use crate::types::Type;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TypeDefinitionCompiler {
    type_compiler: Arc<TypeCompiler>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl TypeDefinitionCompiler {
    pub fn new(
        type_compiler: Arc<TypeCompiler>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
    ) -> Arc<Self> {
        Self {
            type_compiler,
            reference_type_resolver,
        }
        .into()
    }

    pub fn compile(&self, module: &Module) -> Result<Vec<eir::ir::TypeDefinition>, CompileError> {
        Ok(self
            .collect_variant_types(module)?
            .iter()
            .map(|type_| self.compile_type_definitions(type_))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    fn collect_variant_types(&self, module: &Module) -> Result<HashSet<Type>, CompileError> {
        let mut types = HashSet::new();

        module.transform_types(&mut |type_| -> Result<Type, CompileError> {
            types.insert(type_.clone());

            Ok(type_.clone())
        })?;

        Ok(types)
    }

    fn compile_type_definitions(
        &self,
        type_: &Type,
    ) -> Result<Vec<eir::ir::TypeDefinition>, CompileError> {
        Ok(match &self.reference_type_resolver.resolve(type_)? {
            Type::List(list) => vec![eir::ir::TypeDefinition::new(
                self.type_compiler.compile_list(list)?.name(),
                eir::types::Record::new(vec![self.type_compiler.compile_any_list().into()]),
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
}
