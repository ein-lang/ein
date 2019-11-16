use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DependencyPackage {
    version: String,
}
