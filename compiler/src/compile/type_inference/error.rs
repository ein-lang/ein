use crate::debug::SourceInformation;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeInferenceErrorKind {
    TypesNotMatched,
    VariableNotFound,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInferenceError {
    kind: TypeInferenceErrorKind,
    source_information: Rc<SourceInformation>,
}

impl TypeInferenceError {
    pub fn new(
        kind: TypeInferenceErrorKind,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            kind,
            source_information: source_information.into(),
        }
    }
}

impl Display for TypeInferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}:{}", self.kind, self.source_information)
    }
}

impl Error for TypeInferenceError {
    fn description(&self) -> &str {
        "type inferernce error"
    }
}
