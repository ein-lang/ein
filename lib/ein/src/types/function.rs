use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Function {
    argument: Rc<Type>,
    result: Rc<Type>,
    source_information: Rc<SourceInformation>,
}

impl Function {
    pub fn new(
        argument: impl Into<Type>,
        result: impl Into<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            argument: Rc::new(argument.into()),
            result: Rc::new(result.into()),
            source_information: source_information.into(),
        }
    }

    pub fn argument(&self) -> &Type {
        &self.argument
    }

    pub fn result(&self) -> &Type {
        &self.result
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
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
            Type::Function(function) => function.result(),
            _ => &self.result,
        }
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        Self::new(
            self.argument.convert_types(convert),
            self.result.convert_types(convert),
            self.source_information.clone(),
        )
    }
}
