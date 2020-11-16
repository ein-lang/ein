use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use crate::types::Type;
use std::sync::Arc;

pub struct LastResultTypeCalculator {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl LastResultTypeCalculator {
    pub fn new(reference_type_resolver: Arc<ReferenceTypeResolver>) -> Arc<Self> {
        Self {
            reference_type_resolver,
        }
        .into()
    }

    pub fn calculate(&self, type_: &Type, argument_count: usize) -> Result<Type, CompileError> {
        Ok(if argument_count == 0 {
            type_.clone()
        } else {
            self.calculate(
                self.reference_type_resolver
                    .resolve(type_)?
                    .to_function()
                    .unwrap()
                    .result(),
                argument_count - 1,
            )?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::debug::*;
    use crate::types;

    fn create_last_result_type_calculator() -> Arc<LastResultTypeCalculator> {
        LastResultTypeCalculator::new(ReferenceTypeResolver::new(&Module::dummy()))
    }

    #[test]
    fn last_result() {
        assert_eq!(
            create_last_result_type_calculator().calculate(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                1,
            ),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );

        assert_eq!(
            create_last_result_type_calculator().calculate(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),
                2
            ),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );

        assert_eq!(
            create_last_result_type_calculator().calculate(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),
                1
            ),
            Ok(types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into())
        );

        assert_eq!(
            create_last_result_type_calculator().calculate(
                &types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),
                3
            ),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }
}
