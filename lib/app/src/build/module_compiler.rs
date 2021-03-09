use super::error::BuildError;
use super::module_parser::ModuleParser;
use crate::common::{
    FilePath, FilePathConfiguration, FilePathResolver, PackageConfiguration, Target,
};
use crate::infra::{FileSystem, Logger};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct ModuleCompiler<'a> {
    module_parser: &'a ModuleParser<'a>,
    file_path_resolver: &'a FilePathResolver<'a>,
    file_system: &'a dyn FileSystem,
    logger: &'a dyn Logger,
    compile_configuration: Arc<lang::CompileConfiguration>,
    file_path_configuration: &'a FilePathConfiguration,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(
        module_parser: &'a ModuleParser<'a>,
        file_path_resolver: &'a FilePathResolver<'a>,
        file_system: &'a dyn FileSystem,
        logger: &'a dyn Logger,
        compile_configuration: Arc<lang::CompileConfiguration>,
        file_path_configuration: &'a FilePathConfiguration,
    ) -> Self {
        Self {
            module_parser,
            file_path_resolver,
            file_system,
            logger,
            compile_configuration,
            file_path_configuration,
        }
    }

    pub fn compile(
        &self,
        source_file_path: &FilePath,
        module_interfaces: &HashMap<lang::UnresolvedModulePath, lang::ModuleInterface>,
        prelude_module_interfaces: &[lang::ModuleInterface],
        package_configuration: &PackageConfiguration,
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        let source = self.file_system.read_to_string(source_file_path)?;
        let module = self.module_parser.parse(&source, &source_file_path)?;

        let imported_module_interfaces = module
            .imports()
            .iter()
            .map(|import| {
                Ok(module_interfaces
                    .get(import.module_path())
                    .ok_or(BuildError::ModuleNotFound {
                        module_path: import.module_path().clone(),
                    })?
                    .clone())
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

        let module_id =
            self.generate_module_id(source_file_path, &source, &imported_module_interfaces);
        let object_file_path = self.file_path_resolver.resolve_object_file_path(&module_id);
        let interface_file_path = self
            .file_path_resolver
            .resolve_interface_file_path(&module_id);

        if self.file_system.exists(&object_file_path) {
            return Ok((object_file_path, interface_file_path));
        }

        let module_path = self.file_path_resolver.resolve_module_path(
            &source_file_path.relative_to(package_configuration.directory_path()),
            package_configuration.package(),
        );

        self.logger.log(&format!(
            "compiling module {}",
            &module_path.external_unresolved()
        ))?;

        let (module_object_data, module_interface) =
            lang::compile(
                &module.resolve(
                    module_path.clone(),
                    imported_module_interfaces
                        .into_iter()
                        .map(|module_interface| lang::Import::new(module_interface, true))
                        .chain(prelude_module_interfaces.iter().map(|module_interface| {
                            lang::Import::new(module_interface.clone(), false)
                        }))
                        .collect(),
                ),
                // TODO Refactor this by creating the following classes.
                // - MainModuleCompiler
                // - ApplicationPackageBuilder
                // - LibraryPackageBuilder
                if self.is_main_module(&module_path, package_configuration) {
                    self.compile_configuration.clone()
                } else {
                    let mut configuration = self.compile_configuration.as_ref().clone();
                    configuration.main_module_configuration = None;
                    configuration.into()
                },
            )?;

        self.file_system
            .write(&object_file_path, &module_object_data)?;
        self.file_system.write(
            &interface_file_path,
            &serde_json::to_string(&module_interface)?.as_bytes(),
        )?;

        Ok((object_file_path, interface_file_path))
    }

    fn generate_module_id<'b>(
        &self,
        source_file_path: &FilePath,
        source: &str,
        imported_module_interfaces: impl IntoIterator<Item = &'b lang::ModuleInterface>,
    ) -> String {
        let mut hasher = DefaultHasher::new();

        source_file_path.hash(&mut hasher);
        source.hash(&mut hasher);

        for module_interface in imported_module_interfaces {
            module_interface.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    fn is_main_module(
        &self,
        module_path: &lang::ModulePath,
        package_configuration: &PackageConfiguration,
    ) -> bool {
        matches!(
            package_configuration.build_configuration().target(),
            Target::Application(_)
        ) && module_path.components().collect::<Vec<&str>>()
            == vec![self.file_path_configuration.main_file_basename]
    }
}
