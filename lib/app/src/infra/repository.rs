pub struct Repository {
    url: url::Url,
    version: String,
}

impl Repository {
    pub fn new(url: url::Url, version: impl Into<String>) -> Self {
        Self {
            url,
            version: version.into(),
        }
    }

    pub fn url(&self) -> &url::Url {
        &self.url
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
