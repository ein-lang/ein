use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::Type;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct UnionTagCalculator {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl UnionTagCalculator {
    pub fn new(reference_type_resolver: Arc<ReferenceTypeResolver>) -> Arc<Self> {
        Self {
            reference_type_resolver,
        }
        .into()
    }

    pub fn calculate(&self, type_: &Type) -> Result<u64, CompileError> {
        let type_ = self.reference_type_resolver.resolve(type_)?;

        Ok(match type_ {
            Type::Boolean(_)
            | Type::Function(_)
            | Type::List(_)
            | Type::None(_)
            | Type::Number(_)
            | Type::Record(_) => self.convert_type_id_to_tag(&self.calculate_type_id(&type_)?),
            Type::Any(_)
            | Type::Reference(_)
            | Type::Union(_)
            | Type::Unknown(_)
            | Type::Variable(_) => unreachable!(),
        })
    }

    fn calculate_type_id(&self, type_: &Type) -> Result<String, CompileError> {
        // Any and union types can be list element types.
        Ok(match self.reference_type_resolver.resolve(type_)? {
            Type::Any(_) => "Any".into(),
            Type::Boolean(_) => "Boolean".into(),
            Type::Function(function) => format!(
                "({}->{})",
                self.calculate_type_id(function.argument())?,
                self.calculate_type_id(function.result())?
            ),
            Type::List(list) => format!("[{}]", self.calculate_type_id(list.element())?),
            Type::None(_) => "None".into(),
            Type::Number(_) => "Number".into(),
            Type::Record(record) => record.name().into(),
            Type::Union(union) => union
                .types()
                .iter()
                .map(|type_| self.calculate_type_id(type_))
                .collect::<Result<Vec<_>, _>>()?
                .join("|"),
            Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }

    fn convert_type_id_to_tag(&self, type_id: &str) -> u64 {
        // TODO Use safer Hasher.
        let mut hasher = DefaultHasher::new();

        type_id.hash(&mut hasher);

        hasher.finish()
    }
}
