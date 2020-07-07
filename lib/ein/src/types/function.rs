use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Function {
    argument: Arc<Type>,
    result: Arc<Type>,
    source_information: Arc<SourceInformation>,
}

impl Function {
    pub fn new(
        argument: impl Into<Type>,
        result: impl Into<Type>,
        source_information: impl Into<Arc<SourceInformation>>,
    ) -> Self {
        Self {
            argument: Arc::new(argument.into()),
            result: Arc::new(result.into()),
            source_information: source_information.into(),
        }
    }

    pub fn argument(&self) -> &Type {
        &self.argument
    }

    pub fn result(&self) -> &Type {
        &self.result
    }

    pub fn source_information(&self) -> &Arc<SourceInformation> {
        &self.source_information
    }

    pub fn arguments(&self) -> Vec<&Type> {
        let mut arguments: Vec<&Type> = vec![&self.argument];
        let mut result: &Type = &self.result;

        while let Type::Function(function) = result {
            arguments.push(&function.argument);
            result = &function.result;
        }

        arguments
    }

    pub fn last_result(&self) -> &Type {
        match self.result.as_ref() {
            Type::Function(function) => function.last_result(),
            _ => &self.result,
        }
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.argument.convert_types(convert)?,
            self.result.convert_types(convert)?,
            self.source_information.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::debug::*;
    use crate::types;

    #[test]
    fn last_result() {
        assert_eq!(
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .last_result(),
            &types::Number::new(SourceInformation::dummy()).into()
        );

        assert_eq!(
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )
            .last_result(),
            &types::Number::new(SourceInformation::dummy()).into()
        );

        assert_eq!(
            types::Function::new(
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
            .last_result(),
            &types::Number::new(SourceInformation::dummy()).into()
        );
    }
}
