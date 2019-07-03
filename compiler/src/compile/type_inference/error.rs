use crate::debug::SourceInformation;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeInferenceErrorKind {
    TypesNotMatched,
    VariableNotFound,
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
        write!(
            formatter,
            "TypeInferenceError: {}\n{}",
            self.kind, self.source_information
        )
    }
}

impl Error for TypeInferenceError {}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInferenceError {
    kind: TypeInferenceErrorKind,
    source_information: Rc<SourceInformation>,
}

impl Display for TypeInferenceErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                TypeInferenceErrorKind::TypesNotMatched => "Types do not match",
                TypeInferenceErrorKind::VariableNotFound => "Variable not found",
            }
        )
    }
}
