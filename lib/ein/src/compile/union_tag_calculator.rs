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
        Ok(self.transform_type_id_to_tag(&self.calculate_type_id(&type_)?))
    }

    fn transform_type_id_to_tag(&self, type_id: &str) -> u64 {
        // TODO Use safer Hasher.
        let mut hasher = DefaultHasher::new();

        type_id.hash(&mut hasher);

        hasher.finish()
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
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;

    #[test]
    fn calculate_none_list_type_id() {
        assert_eq!(
            UnionTagCalculator::new(ReferenceTypeResolver::new(&Module::dummy()))
                .calculate_type_id(
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
            UnionTagCalculator::new(ReferenceTypeResolver::new(&Module::dummy()))
                .calculate_type_id(
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
            UnionTagCalculator::new(ReferenceTypeResolver::new(&Module::dummy()))
                .calculate_type_id(
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
