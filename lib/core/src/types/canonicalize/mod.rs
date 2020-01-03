mod type_canonicalizer;
mod type_equality_checker;

use crate::types::Type;
use type_canonicalizer::TypeCanonicalizer;

pub fn canonicalize(type_: &Type) -> Type {
    TypeCanonicalizer::new().canonicalize(type_)
}
