use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DependencyPackage {
    name: String,
    version: String,
}
