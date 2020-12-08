mod build_configuration;
mod command_target;
mod external_package_configuration;
mod target;
mod target_type;

use crate::infra::FilePath;
pub use build_configuration::BuildConfiguration;
pub use command_target::CommandTarget;
pub use external_package_configuration::ExternalPackageConfiguration;
pub use target::Target;

#[derive(Clone, Debug)]
pub struct PackageConfiguration {
    pub package: ein::Package,
    pub build_configuration: BuildConfiguration,
    pub directory_path: FilePath,
}
