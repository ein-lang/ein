mod json_application_build_configuration;
mod json_external_package_configuration;
mod json_system_package_configuration;

use crate::{
    common::ApplicationTarget, common::BuildConfiguration, ExternalPackage, SystemPackage, Target,
};
use json_application_build_configuration::JsonApplicationBuildConfiguration;
use json_external_package_configuration::JsonExternalPackageConfiguration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::json_system_package_configuration::JsonSystemPackageConfiguration;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonBuildConfiguration {
    application: Option<JsonApplicationBuildConfiguration>,
    dependencies: HashMap<String, JsonExternalPackageConfiguration>,
}

impl JsonBuildConfiguration {
    pub fn new(
        application: Option<JsonApplicationBuildConfiguration>,
        dependencies: HashMap<String, JsonExternalPackageConfiguration>,
    ) -> Self {
        Self {
            application,
            dependencies,
        }
    }

    pub fn serialize(configuration: &BuildConfiguration) -> Self {
        JsonBuildConfiguration::new(
            match configuration.target() {
                Target::Application(application) => Some(JsonApplicationBuildConfiguration::new(
                    application.name(),
                    JsonSystemPackageConfiguration::new(
                        application.system_package().name(),
                        application.system_package().version(),
                    ),
                )),
                Target::Library => None,
            },
            configuration
                .dependencies()
                .iter()
                .map(|external_package| {
                    (
                        external_package.name().into(),
                        JsonExternalPackageConfiguration::new(external_package.version()),
                    )
                })
                .collect(),
        )
    }

    pub fn deserialize(&self) -> BuildConfiguration {
        BuildConfiguration::new(
            self.application
                .as_ref()
                .map(|application| {
                    ApplicationTarget::new(
                        application.name(),
                        SystemPackage::new(
                            application.system().name(),
                            application.system().version(),
                        ),
                    )
                    .into()
                })
                .unwrap_or(Target::Library),
            self.dependencies
                .iter()
                .map(|(name, configuration)| ExternalPackage::new(name, configuration.version()))
                .collect(),
        )
    }
}
