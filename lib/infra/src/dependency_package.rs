use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExternalPackage {
    version: String,
}
