use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub enum PackageTargetType {
    Binary,
    Library,
}
