use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct ExternalPackageConfiguration {
    pub version: String,
}
