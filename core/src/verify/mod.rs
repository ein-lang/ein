mod error;
mod type_check;

use super::ast::Module;
pub use error::*;
use type_check::check_types;

pub fn verify(module: &Module) -> Result<(), VerificationError> {
    check_types(module)?;

    Ok(())
}
