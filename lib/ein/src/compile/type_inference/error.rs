use crate::debug::SourceInformation;
use crate::types;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeInferenceError {
    TypesNotMatched(Rc<SourceInformation>, Rc<SourceInformation>),
    TypeNotFound { reference: types::Reference },
    VariableNotFound(String, Rc<SourceInformation>),
}

impl TypeInferenceError {}

impl Display for TypeInferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TypeInferenceError::TypesNotMatched(lhs_source_information, rhs_source_information) => {
                write!(
                    formatter,
                    "types not matched\n{}\n{}",
                    lhs_source_information, rhs_source_information
                )
            }
            TypeInferenceError::TypeNotFound { reference } => write!(
                formatter,
                "type \"{}\" not found\n{}",
                reference.name(),
                reference.source_information()
            ),
            TypeInferenceError::VariableNotFound(name, source_information) => write!(
                formatter,
                "variable \"{}\" not found\n{}",
                name, source_information
            ),
        }
    }
}

impl Error for TypeInferenceError {}
