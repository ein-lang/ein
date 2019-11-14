use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DependencyPackage {
    url: String,
    version: String,
}
