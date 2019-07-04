use crate::debug::SourceInformation;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeInferenceError {
    TypesNotMatched(Rc<SourceInformation>, Rc<SourceInformation>),
    VariableNotFound(Rc<SourceInformation>),
}

impl TypeInferenceError {}

impl Display for TypeInferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            TypeInferenceError::TypesNotMatched(lhs_source_information, rhs_source_information) => {
                write!(
                    formatter,
                    "TypeInferenceError: Types do not match\n{}\n{}",
                    lhs_source_information, rhs_source_information
                )
            }
            TypeInferenceError::VariableNotFound(source_information) => write!(
                formatter,
                "TypeInferenceError: Variable not found\n{}",
                source_information
            ),
        }
    }
}

impl Error for TypeInferenceError {}
