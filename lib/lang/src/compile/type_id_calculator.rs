use super::{error::CompileError, reference_type_resolver::ReferenceTypeResolver};
use crate::types::Type;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub struct TypeIdCalculator {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl TypeIdCalculator {
    pub fn new(reference_type_resolver: Arc<ReferenceTypeResolver>) -> Arc<Self> {
        Self {
            reference_type_resolver,
        }
        .into()
    }

    pub fn calculate(&self, type_: &Type) -> Result<u64, CompileError> {
        // TODO Use safer Hasher.
        let mut hasher = DefaultHasher::new();

        self.calculate_type_id(&type_)?.hash(&mut hasher);

        Ok(hasher.finish())
    }

    fn calculate_type_id(&self, type_: &Type) -> Result<String, CompileError> {
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
            Type::String(_) => "String".into(),
            Type::Union(union) => format!(
                "({})",
                union
                    .types()
                    .iter()
                    .map(|type_| self.calculate_type_id(type_))
                    .collect::<Result<Vec<_>, _>>()?
                    .join("|")
            ),
            Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, debug::SourceInformation, types};

    #[test]
    fn calculate_none_list_type_id() {
        assert_eq!(
            TypeIdCalculator::new(ReferenceTypeResolver::new(&Module::dummy())).calculate_type_id(
                &types::List::new(
                    types::None::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok("[None]".into())
        );
    }

    #[test]
    fn calculate_any_list_type_id() {
        assert_eq!(
            TypeIdCalculator::new(ReferenceTypeResolver::new(&Module::dummy())).calculate_type_id(
                &types::List::new(
                    types::Any::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok("[Any]".into())
        );
    }

    #[test]
    fn calculate_union_list_type_id() {
        assert_eq!(
            TypeIdCalculator::new(ReferenceTypeResolver::new(&Module::dummy())).calculate_type_id(
                &types::List::new(
                    types::Union::new(
                        vec![
                            types::Number::new(SourceInformation::dummy()).into(),
                            types::None::new(SourceInformation::dummy()).into(),
                        ],
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into()
            ),
            Ok("[(None|Number)]".into())
        );
    }
}
